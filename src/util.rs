// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Utility code that is shared by multiple subcommands;

use crate::error::{Result, ToolErrorKind};
use log::{error, info};
use parsec_client::core::interface::operations::psa_algorithm::{Algorithm, Hash, SignHash};
use parsec_client::BasicClient;
use picky_asn1::wrapper::IntegerAsn1;
use serde::{Deserialize, Serialize};
use sha2::digest::{Digest, DynDigest};

#[derive(Serialize, Deserialize)]
struct EccSignature {
    r: IntegerAsn1,
    s: IntegerAsn1,
}

/// Signs a given message using the hashing and signing policy that was associated with the given key when
/// it was created.
///
/// If the signing key allows for the use of any hashing algorithm, then a default hash can optionally be passed
/// by the caller, and this hash will be used (otherwise the function will fail).
pub fn sign_message_with_policy(
    basic_client: &BasicClient,
    key_name: &str,
    msg: &[u8],
    default_hash: Option<Hash>,
) -> Result<Vec<u8>> {
    let alg = basic_client
        .key_attributes(key_name)?
        .policy
        .permitted_algorithms;

    let signature = match alg {
        Algorithm::AsymmetricSignature(alg) => {
            let hash = match alg.hash() {
                Some(SignHash::Specific(hash)) => hash_data(msg, hash)?,
                Some(SignHash::Any) => {
                    if let Some(hash) = default_hash {
                        hash_data(msg, hash)?
                    } else {
                        error!("Signing key allows any hashing algorithm, but no default was specified.");
                        return Err(ToolErrorKind::NotSupported.into());
                    }
                }
                _ => {
                    error!("Asymmetric signing algorithm ({:?}) is not supported", alg);
                    return Err(ToolErrorKind::NotSupported.into());
                }
            };
            info!("Signing data with {:?}...", alg);
            let mut sig = basic_client.psa_sign_hash(key_name, &hash, alg)?;
            if alg.is_ecc_alg() {
                let s = IntegerAsn1::from_bytes_be_unsigned(sig.split_off(sig.len() / 2));
                sig = picky_asn1_der::to_vec(&EccSignature {
                    r: IntegerAsn1::from_bytes_be_unsigned(sig),
                    s,
                })
                .unwrap();
            }

            sig
        }
        other => {
            error!(
                "Key's algorithm is {:?} which can not be used for signing.",
                other
            );
            return Err(ToolErrorKind::WrongKeyAlgorithm.into());
        }
    };

    Ok(signature)
}

fn hash_data(data: &[u8], alg: Hash) -> Result<Vec<u8>> {
    let mut hasher: Box<dyn DynDigest> = match alg {
        Hash::Sha224 => Box::from(sha2::Sha224::new()),
        Hash::Sha256 => Box::from(sha2::Sha256::new()),
        Hash::Sha384 => Box::from(sha2::Sha384::new()),
        Hash::Sha512 => Box::from(sha2::Sha512::new()),
        _ => {
            error!("Hashing algorithm ({:?}) not supported", alg);
            return Err(ToolErrorKind::NotSupported.into());
        }
    };
    info!("Hashing data with {:?}...", alg);
    hasher.update(data);
    Ok(hasher.finalize().to_vec())
}
