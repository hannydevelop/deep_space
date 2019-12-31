use crate::msg::Msg;
use signatory::ecdsa::{FixedSignature,curve::Secp256k1};
use crate::stdfee::StdFee;
use serde::{Serialize, Serializer};
use std::fmt;

pub enum TxSignature{
    StandardSDKSignature(FixedSignature<Secp256k1>)
}

impl fmt::Debug for TxSignature{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            TxSignature::StandardSDKSignature(sig) => Ok(write!(f, "ECDSA Signature: {}", base64::encode(sig))?),
        }
    }
}

impl Serialize for TxSignature{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self{
            TxSignature::StandardSDKSignature(sig) => serializer.serialize_str(&base64::encode(sig)),
        }
    }
}    


/// An enum that bundles the signed transaction with signatures.
#[derive(Serialize, Default, Debug)]
pub struct StdTx {
    pub msg: Vec<Msg>,
    pub fee: StdFee,
    pub memo: String,
    pub signatures: Vec<TxSignature>,
}


