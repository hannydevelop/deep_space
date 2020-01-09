//! Message format for broadcasting transctions
//!

use crate::msg::Msg;
use crate::stdfee::StdFee;
use serde::{Serialize, Serializer};
use signatory::ecdsa::{curve::Secp256k1, FixedSignature};
use std::fmt;

/// Signatures on the transactions
pub enum TxSignature {
    /// Standard Secp256k1 ECDSA signature
    StandardSDKSignature(FixedSignature<Secp256k1>),
}

impl fmt::Debug for TxSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TxSignature::StandardSDKSignature(sig) => {
                Ok(write!(f, "ECDSA Signature: {}", base64::encode(sig))?)
            }
        }
    }
}

impl Serialize for TxSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TxSignature::StandardSDKSignature(sig) => {
                serializer.serialize_str(format!("\"signature\":{}", &base64::encode(sig)).as_ref())
            }
        }
    }
}

/// An enum that bundles the signed transaction with signatures.
#[derive(Serialize, Default, Debug)]
pub struct StdTx {
    /// Messages in transction
    pub msg: Vec<Msg>,
    /// Fees to be paid
    pub fee: StdFee,
    /// Memo field
    pub memo: String,
    /// Signatures
    pub signatures: Vec<TxSignature>,
}

/// Cosmos SDk transaction wrapper
#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum CosmosSDKTx{
    /// Standard Cosmos SDK transaction interface
    #[serde(rename = "cosmos-sdk/StdTx")]
    CosmosStdTx(StdTx),
    /// Terra Cosmos SDK transaction interface
    #[serde(rename = "core/StdTx")]
    Core(StdTx),

}
