// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Signs data.
//!
//! Will use the algorithm set to the key's policy during creation.

use crate::error::{Result, ToolErrorKind};
use log::{error, info};
use parsec_client::core::interface::operations::psa_algorithm::{Algorithm, Hash, SignHash};
use parsec_client::BasicClient;
use picky_asn1::wrapper::IntegerAsn1;
use serde::{Deserialize, Serialize};
use sha2::digest::{Digest, DynDigest};
use structopt::StructOpt;

/// Signs data.
#[derive(Debug, StructOpt)]
pub struct Sign {
    #[structopt(short = "k", long = "key-name")]
    key_name: String,

    /// String of UTF-8 text
    input_data: String,
}

#[derive(Serialize, Deserialize)]
struct EccSignature {
    r: IntegerAsn1,
    s: IntegerAsn1,
}

impl Sign {
    /// Signs data.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let alg = basic_client
            .key_attributes(&self.key_name)?
            .policy
            .permitted_algorithms;

        let signature = match alg {
            Algorithm::AsymmetricSignature(alg) => {
                let hash = match alg.hash() {
                    Some(SignHash::Specific(hash)) => hash_data(self.input_data.as_bytes(), hash)?,
                    _ => {
                        error!("Asymmetric signing algorithm ({:?}) is not supported", alg);
                        return Err(ToolErrorKind::NotSupported.into());
                    }
                };
                info!("Signing data with {:?}...", alg);
                let mut sig = basic_client.psa_sign_hash(self.key_name.clone(), &hash, alg)?;
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

        let signature = base64::encode(&signature);

        println!("{}", signature);

        Ok(())
    }
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
    hasher.update(&data);
    Ok(hasher.finalize().to_vec())
}
