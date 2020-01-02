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
    pub fn generate_vote_hash(&self) -> String {
        let data = format!(
            "{}:{}:{}:{}",
            self.salt,
            self.exchange_rate,
            self.denom,
            self.validator.to_bech32("terravaloper")
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
pub struct MsgDelegateFeedConsent {
    pub operator: Address,
    pub feeder: Address,
}

/// Any arbitrary message
#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Msg {
    #[serde(rename = "cosmos-sdk/MsgSend")]
    SendMsg(SendMsg),
    #[serde(rename = "oracle/MsgExchangeRateVote")]
    MsgExchangeRateVote(MsgExchangeRateVote),
    #[serde(rename = "oracle/MsgExchangeRatePrevote")]
    MsgExchangeRatePrevote(MsgExchangeRatePrevote),
    #[serde(rename = "oracle/MsgDelegateFeedConsent")]
    MsgDelegateFeedConsent(MsgDelegateFeedConsent),
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
    use super::{Msg, MsgExchangeRateVote, Address};
    use serde_json::{from_str, json, to_string, Value};
    use rust_decimal::Decimal;
    #[test]
    fn test_serialize_msg() {
        let msg: Msg = Msg::Test("TestMsg1".to_string());
        let s = to_string(&msg).expect("Unable to serialize");
        let v: Value = from_str(&s).expect("Unable to deserialize");
        assert_eq!(v, json!({"type": "deep_space/Test", "value": "TestMsg1"}));
    }
    #[test]
    fn test_has_prevote_msg() {
        let vote: Msg = Msg::MsgExchangeRateVote(MsgExchangeRateVote{
            exchange_rate: Decimal::new(-100,2),
            denom:"test".to_string(),
            salt:"hello_world".to_string(),
            feeder: Address::from_bech32("terra1grgelyng2v6v3t8z87wu3sxgt9m5s03x259evd").unwrap(),
            validator: Address::from_bech32("terravaloper1grgelyng2v6v3t8z87wu3sxgt9m5s03x2mfyu7").unwrap(),

        });

        let mut hash_str: String = "".to_string();
        if let Msg::MsgExchangeRateVote(x) =vote{
            hash_str = x.generate_vote_hash();
        }
        assert_ne!(hash_str,"");
    }
}
