use crate::address::Address;
use crate::canonical_json::to_canonical_json;
use crate::coin::Coin;
use failure::Error;
use rust_decimal::Decimal;
use serde::Serialize;
use sha2::{digest::Digest, Sha256};
use subtle_encoding::hex;

#[derive(Serialize, Debug, Clone)]
pub struct SendMsg {
    pub from_address: Address,
    pub to_address: Address,
    pub amount: Vec<Coin>,
}

#[derive(Serialize, Debug, Clone)]
pub struct MsgExchangeRateVote {
    pub exchange_rate: Decimal,
    pub salt: String,
    pub denom: String,
    pub feeder: Address,
    pub validator: Address,
}

impl MsgExchangeRateVote {
    /// Generatea hex encoded truncated sha256 of vote. Needed to generate prevote
    fn generate_vote_hash(&self) -> String {
        let data = format!(
            "{}:{}:{}:{}",
            self.salt,
            self.exchange_rate,
            self.denom,
            self.validator.to_bech32("cosmosvalop")
        );
        //Tendermint truncated sha256
        let digest = Sha256::digest(data.as_bytes());
        let mut bytes = [0u8; 20];
        bytes.copy_from_slice(&digest[..20]);
        // Should always succeed.
        String::from_utf8(hex::encode(bytes)).unwrap()
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct MsgExchangeRatePrevote {
    pub hash: String,
    pub denom: String,
    pub feeder: Address,
    pub validator: Address,
}

#[derive(Serialize, Debug, Clone)]
pub struct MsgDelegate {
    pub operator: Address,
    pub feeder: Address,
}

/// Any arbitrary message
#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Msg {
    #[serde(rename = "cosmos-sdk/MsgSend")]
    SendMsg(SendMsg),
    #[serde(rename = "deep_space/Test")]
    Test(String),
}

impl Msg {
    pub fn to_sign_bytes(self) -> Result<Vec<u8>, Error> {
        Ok(to_canonical_json(self)?)
    }
}

#[cfg(test)]
mod tests {
    use super::Msg;
    use serde_json::{from_str, json, to_string, Value};
    #[test]
    fn test_serialize_msg() {
        let msg: Msg = Msg::Test("TestMsg1".to_string());
        let s = to_string(&msg).expect("Unable to serialize");
        let v: Value = from_str(&s).expect("Unable to deserialize");
        assert_eq!(v, json!({"type": "deep_space/Test", "value": "TestMsg1"}));
    }
}
