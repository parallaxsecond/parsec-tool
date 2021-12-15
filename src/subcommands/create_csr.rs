// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Creates a Certificate Signing Request (CSR) from a keypair.

use crate::error::{Error, Result, ToolErrorKind};
use crate::util::sign_message_with_policy;
use log::error;
use parsec_client::core::interface::operations::psa_algorithm::{
    Algorithm, AsymmetricSignature, Hash, SignHash,
};
use parsec_client::core::interface::operations::psa_key_attributes::{EccFamily, Type};
use parsec_client::BasicClient;
use rcgen::{
    Certificate, CertificateParams, DistinguishedName, DnType, KeyPair, RcgenError, RemoteKeyPair,
    SignatureAlgorithm, PKCS_ECDSA_P256_SHA256, PKCS_ECDSA_P384_SHA384, PKCS_RSA_SHA256,
    PKCS_RSA_SHA384, PKCS_RSA_SHA512,
};
use structopt::StructOpt;

/// Creates an X509 Certificate Signing Request (CSR) from a keypair, using the signing algorithm
/// that is associated with the key.
///
/// The CSR is written to the standard output in PEM format by default.
#[derive(Debug, StructOpt)]
pub struct CreateCsr {
    /// The name of the key to use for signing. This must be an existing key that is accessible
    /// to the user, and it must be a signing key (either an RSA key or an elliptic curve key).
    ///
    /// Elliptic curve keys must use the NIST P256 or P384 curves.
    #[structopt(short = "k", long = "key-name")]
    key_name: String,

    /// The common name to be used within the Distinguished Name (DN) specification of
    /// the CSR.
    #[structopt(long = "cn")]
    common_name: Option<String>,

    /// The locality name to be used within the Distinguished Name (DN) specification of
    /// the CSR.
    #[structopt(long = "l")]
    locality: Option<String>,

    /// The organization name to be used within the Distinguished Name (DN) specification of
    /// the CSR.
    #[structopt(long = "o")]
    organization: Option<String>,

    /// The organizational unit name to be used within the Distinguished Name (DN) specification
    /// of the CSR.
    #[structopt(long = "ou")]
    organizational_unit: Option<String>,

    /// The state name to be used within the Distinguished Name (DN) specification of the CSR.
    #[structopt(long = "st")]
    state: Option<String>,

    /// The country name to be used within the Distinguished Name (DN) specification of the CSR.
    #[structopt(long = "c")]
    country: Option<String>,

    /// A Subject Alternative Name (SAN) for the domain of the CSR.
    #[structopt(long = "san")]
    subject_alternative_name: Option<Vec<String>>,
}

/// Short-lived structure to encapsulate the key name and the client, so that we can implement the
/// RemoteKeyPair trait for rcgen.
struct ParsecRemoteKeyPair {
    key_name: String,
    public_key_der: Vec<u8>,
    parsec_client: BasicClient,
    rcgen_algorithm: &'static SignatureAlgorithm,
}

impl CreateCsr {
    /// Creates a Certificate Signing Request (CSR) from a keypair.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let public_key = basic_client.psa_export_public_key(&self.key_name)?;

        let rcgen_algorithm = self.get_rcgen_algorithm(&basic_client)?;

        let parsec_key_pair = ParsecRemoteKeyPair {
            key_name: self.key_name.clone(),
            public_key_der: public_key,
            // "Move" the client into the struct here.
            parsec_client: basic_client,
            rcgen_algorithm,
        };

        let remote_key_pair = KeyPair::from_remote(Box::new(parsec_key_pair))?;

        let subject_alt_names = match &self.subject_alternative_name {
            Some(san) => san.to_owned(),
            None => Vec::new(),
        };

        let mut dn = DistinguishedName::new();

        if let Some(common_name) = &self.common_name {
            dn.push(DnType::CommonName, common_name.clone());
        }

        if let Some(organizational_unit) = &self.organizational_unit {
            // NOTE: X509 permits multiple OUs, but the RCGEN crate only preserves one entry, so for now the
            // parsec-tool also only accepts one entry on the command-line. If this changes in the future, it
            // will be possible to evolve the command-line parser to accept multiple values without it being
            // a breaking change.
            dn.push(DnType::OrganizationalUnitName, organizational_unit.clone());
        }

        if let Some(organization) = &self.organization {
            dn.push(DnType::OrganizationName, organization.clone());
        }

        if let Some(locality) = &self.locality {
            dn.push(DnType::LocalityName, locality.clone());
        }

        if let Some(state) = &self.state {
            dn.push(DnType::StateOrProvinceName, state.clone());
        }

        if let Some(country) = &self.country {
            dn.push(DnType::CountryName, country.clone());
        }

        let mut params = CertificateParams::new(subject_alt_names);
        params.alg = rcgen_algorithm;
        params.key_pair = Some(remote_key_pair);
        params.distinguished_name = dn;

        let cert = Certificate::from_params(params)?;

        let pem_string = cert.serialize_request_pem()?;

        println!("{}", pem_string);

        Ok(())
    }

    // Inspect the attributes of the signing key and map them down to one of rcgen's supported hash-and-sign
    // schemes (throwing an error if there isn't a suitable mapping).
    //
    // There's rather a lot of complexity here, because we need to map down lots of nested PSA properties onto a small number
    // of hash-and-sign schemes that RCGEN supports.
    fn get_rcgen_algorithm(
        &self,
        basic_client: &BasicClient,
    ) -> Result<&'static SignatureAlgorithm> {
        let attributes = basic_client.key_attributes(&self.key_name)?;

        if let Algorithm::AsymmetricSignature(alg) = attributes.policy.permitted_algorithms {
            match alg {
                AsymmetricSignature::RsaPkcs1v15Sign { hash_alg } => match hash_alg {
                    SignHash::Specific(Hash::Sha256) => Ok(&PKCS_RSA_SHA256),
                    SignHash::Specific(Hash::Sha384) => Ok(&PKCS_RSA_SHA384),
                    SignHash::Specific(Hash::Sha512) => Ok(&PKCS_RSA_SHA512),
                    SignHash::Any => Ok(&PKCS_RSA_SHA256), // Default hash algorithm for the tool.
                    _ => {
                        // The algorithm is specific, but not one that RCGEN can use, so fail the operation.
                        error!("Signing key requires use of hashing algorithm ({:?}), which is not supported for certificate requests.", alg);
                        Err(ToolErrorKind::NotSupported.into())
                    }
                },
                AsymmetricSignature::RsaPkcs1v15SignRaw => {
                    // Key policy specifies raw RSA signatures. RCGEN will always hash-and-sign, so fail.
                    error!("Signing key specifies raw signing only, which is not supported for certificate requests.");
                    Err(ToolErrorKind::NotSupported.into())
                }
                AsymmetricSignature::RsaPss { .. } => {
                    error!("Signing key specifies RSA PSS scheme, which is not supported for certificate requests.");
                    Err(ToolErrorKind::NotSupported.into())
                }
                AsymmetricSignature::Ecdsa { hash_alg } => {
                    if !matches!(
                        attributes.key_type,
                        Type::EccKeyPair {
                            curve_family: EccFamily::SecpR1
                        }
                    ) {
                        error!(
                            "Signing key must use curve family SecpR1 for certificate requests."
                        );
                        return Err(ToolErrorKind::NotSupported.into());
                    };

                    match hash_alg {
                        SignHash::Specific(Hash::Sha256) => {
                            if attributes.bits == 256 {
                                Ok(&PKCS_ECDSA_P256_SHA256)
                            } else {
                                error!("Signing key should have strength 256, but actually has strength {}.", attributes.bits);
                                Err(ToolErrorKind::NotSupported.into())
                            }
                        }
                        SignHash::Specific(Hash::Sha384) => {
                            if attributes.bits == 384 {
                                Ok(&PKCS_ECDSA_P384_SHA384)
                            } else {
                                error!("Signing key should have strength 384, but actually has strength {}.", attributes.bits);
                                Err(ToolErrorKind::NotSupported.into())
                            }
                        }
                        SignHash::Any => {
                            match attributes.bits {
                                256 => Ok(&PKCS_ECDSA_P256_SHA256),
                                _ => {
                                    // We have to fail this, because ParsecRemoteKeyPair::sign() defaults the hash to SHA-256, and RCGEN
                                    // doesn't support a hash algorithm that is different from the key strength.
                                    error!("Signing keys of strength other than 256-bit not supported without specific hash algorithm.");
                                    Err(ToolErrorKind::NotSupported.into())
                                }
                            }
                        }
                        _ => {
                            // The algorithm is specific, but not one that RCGEN can use, so fail the operation.
                            error!("Signing key requires use of hashing algorithm ({:?}), which is not supported for certificate requests.", alg);
                            Err(ToolErrorKind::NotSupported.into())
                        }
                    }
                }
                _ => {
                    // Unsupported algorithm.
                    error!("The specified key is not supported for certificate requests.");
                    Err(ToolErrorKind::NotSupported.into())
                }
            }
        } else {
            error!("Specified key is not an asymmetric signing key, which is needed for certificate requests.");
            Err(ToolErrorKind::WrongKeyAlgorithm.into())
        }
    }
}

impl RemoteKeyPair for ParsecRemoteKeyPair {
    fn public_key(&self) -> &[u8] {
        &self.public_key_der
    }

    fn sign(&self, msg: &[u8]) -> std::result::Result<Vec<u8>, RcgenError> {
        let signature =
            sign_message_with_policy(&self.parsec_client, &self.key_name, msg, Some(Hash::Sha256))
                .map_err(RcgenError::from)?;
        Ok(signature)
    }

    fn algorithm(&self) -> &'static SignatureAlgorithm {
        self.rcgen_algorithm
    }
}

impl From<Error> for RcgenError {
    fn from(_e: Error) -> Self {
        // There isn't a suitable mapping, because RcgenError does not have a variant for the
        // case where RemoteKeyPair failed for third-party reasons.
        // See: https://github.com/est31/rcgen/issues/67
        // The crate will publish a new enum variant. When this change is released, we can rework this to be a
        // more suitable error.
        RcgenError::KeyGenerationUnavailable
    }
}
