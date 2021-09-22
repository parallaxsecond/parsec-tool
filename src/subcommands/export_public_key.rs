// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Exports a public key.

use crate::error::{Result, ToolErrorKind};
use log::error;
use oid::prelude::*;
use parsec_client::core::interface::operations::psa_key_attributes::{EccFamily, Type};
use parsec_client::BasicClient;
use picky_asn1::bit_string::BitString;
use picky_asn1_x509::{
    AlgorithmIdentifier, EcParameters, PublicKey, RsaPublicKey, SubjectPublicKeyInfo,
};
use structopt::StructOpt;

/// Exports a PEM-encoded public key.
#[derive(Debug, StructOpt)]
pub struct ExportPublicKey {
    #[structopt(short = "k", long = "key-name")]
    key_name: String,

    /// Export RSA Public Key in PKCS#1 format.
    #[structopt(long = "pkcs1")]
    pkcs1: bool,
}

impl ExportPublicKey {
    /// Exports a public key.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let mut tag = String::from("PUBLIC KEY");
        let psa_public_key = basic_client.psa_export_public_key(&self.key_name)?;
        let mut public_key = psa_public_key.clone();

        match basic_client.key_attributes(&self.key_name)?.key_type {
            Type::RsaKeyPair | Type::RsaPublicKey => {
                if self.pkcs1 {
                    tag = String::from("RSA PUBLIC KEY");
                } else {
                    let pkcs8_public_key = SubjectPublicKeyInfo {
                        algorithm: AlgorithmIdentifier::new_rsa_encryption(),
                        subject_public_key: PublicKey::Rsa(
                            picky_asn1_der::from_bytes::<RsaPublicKey>(&psa_public_key)
                                .unwrap()
                                .into(),
                        ),
                    };
                    public_key = picky_asn1_der::to_vec(&pkcs8_public_key).unwrap();
                }
            }
            Type::EccKeyPair {
                curve_family: curve,
            }
            | Type::EccPublicKey {
                curve_family: curve,
            } => {
                if self.pkcs1 {
                    error!("PKCS1 format doesn't support ECC keys");
                    return Err(ToolErrorKind::WrongKeyAlgorithm.into());
                } else {
                    let key_bits = basic_client.key_attributes(&self.key_name)?.bits;
                    let pkcs8_public_key = SubjectPublicKeyInfo {
                        algorithm: AlgorithmIdentifier::new_elliptic_curve(
                            EcParameters::NamedCurve(curve_oid(curve, key_bits).unwrap().into()),
                        ),
                        subject_public_key: PublicKey::Ec(
                            BitString::with_bytes(psa_public_key).into(),
                        ),
                    };
                    public_key = picky_asn1_der::to_vec(&pkcs8_public_key).unwrap();
                }
            }
            _ => {
                error!("Unsupported type of key");
                return Err(ToolErrorKind::NotSupported.into());
            }
        };

        let pem_encoded = pem::encode_config(
            &pem::Pem {
                tag,
                contents: public_key,
            },
            pem::EncodeConfig {
                line_ending: pem::LineEnding::LF,
            },
        );

        print!("{}", pem_encoded);
        Ok(())
    }
}

fn curve_oid(curve: EccFamily, key_bits: usize) -> Result<ObjectIdentifier> {
    let curve_oid = match curve {
        // SEC random curves over prime fields.
        EccFamily::SecpR1 => match key_bits {
            192 => picky_asn1_x509::oids::secp192r1(),
            224 => picky_asn1_x509::oids::secp224r1(),
            256 => picky_asn1_x509::oids::secp256r1(),
            384 => picky_asn1_x509::oids::secp384r1(),
            521 => picky_asn1_x509::oids::secp521r1(),
            _ => return print_error(curve, key_bits),
        },
        // SEC Koblitz curves over prime fields.
        // OIDs are not defined in picky_asn1_x509::oids and in RFC5480.
        // Use values from https://www.secg.org/sec2-v2.pdf#subsection.A.2
        EccFamily::SecpK1 => match key_bits {
            192 => ObjectIdentifier::try_from("1.3.132.0.31").unwrap(),
            224 => ObjectIdentifier::try_from("1.3.132.0.32").unwrap(),
            256 => ObjectIdentifier::try_from("1.3.132.0.10").unwrap(),
            _ => return print_error(curve, key_bits),
        },
        // SEC Koblitz curves over binary fields
        EccFamily::SectK1 => match key_bits {
            233 => picky_asn1_x509::oids::sect233k1(),
            283 => picky_asn1_x509::oids::sect283k1(),
            409 => picky_asn1_x509::oids::sect409k1(),
            571 => picky_asn1_x509::oids::sect571k1(),
            _ => return print_error(curve, key_bits),
        },
        // SEC random curves over binary fields
        EccFamily::SectR1 => match key_bits {
            233 => picky_asn1_x509::oids::sect233r1(),
            283 => picky_asn1_x509::oids::sect283r1(),
            409 => picky_asn1_x509::oids::sect409r1(),
            571 => picky_asn1_x509::oids::sect571r1(),
            _ => return print_error(curve, key_bits),
        },
        _ => {
            error!("Unsupported Ecc family \"{}\"", curve);
            return Err(ToolErrorKind::NotSupported.into());
        }
    };
    Ok(curve_oid)
}

fn print_error(curve: EccFamily, key_bits: usize) -> Result<ObjectIdentifier> {
    error!(
        "Unsupported number of bits {} for Ecc family \"{}\"",
        key_bits, curve
    );
    Err(ToolErrorKind::NotSupported.into())
}
