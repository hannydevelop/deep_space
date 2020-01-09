//! Transaction MSGs for various comsos sdk chains
//!

use crate::address::{Address, TerraAddress, TerraValidatorOperatorAddress};
use crate::canonical_json::to_canonical_json;
use crate::coin::Coin;
use failure::Error;
use rust_decimal::Decimal;
use serde::Serialize;
use sha2::{digest::Digest, Sha256};
use subtle_encoding::hex;

/// Sends Coins between accounts
#[derive(Serialize, Debug, Clone)]
pub struct SendMsg {
    /// Origin address
    pub from_address: Address,
    /// Destination Address
    pub to_address: Address,
    /// Coins to be sent
    pub amount: Vec<Coin>,
}
/// Terra Oracle Vote Message
#[derive(Serialize, Debug, Clone)]
pub struct MsgExchangeRateVote {
    /// Exchange rate voted on. Negative values are an abstain vote.
    pub exchange_rate: Decimal,
    /// Salt for commit reveal prootocol
    pub salt: String,
    /// Denom for Oracle Vote
    pub denom: String,
    /// Origin of the Feed Msg
    pub feeder: TerraAddress,
    /// Validator voting on behalf of
    pub validator: TerraValidatorOperatorAddress,
}

impl MsgExchangeRateVote {
    /// Generatea hex encoded truncated sha256 of vote. Needed to generate prevote
    pub fn generate_vote_hash(&self) -> String {
        let data = format!(
            "{}:{}:{}:{}",
            self.salt, self.exchange_rate, self.denom, self.validator,
        );
        //Tendermint truncated sha256
        let digest = Sha256::digest(data.as_bytes());
        let mut bytes = [0u8; 20];
        bytes.copy_from_slice(&digest[..20]);
        // Should always succeed.
        String::from_utf8(hex::encode(bytes)).unwrap()
    }
}

/// Terra Oracle Prevote message
#[derive(Serialize, Debug, Clone)]
pub struct MsgExchangeRatePrevote {
    /// Commitment to future vote
    pub hash: String,
    /// Denom to commit for
    pub denom: String,
    /// Origin Address for vote
    pub feeder: TerraAddress,
    /// Validator voting on behalf of
    pub validator: TerraValidatorOperatorAddress,
}

/// Delegate Terra Oracle voting account to a new key
#[derive(Serialize, Debug, Clone)]
pub struct MsgDelegateFeedConsent {
    /// operator delegating voting authority
    pub operator: TerraValidatorOperatorAddress,
    /// key delegated to
    pub feeder: TerraAddress,
}

/// Any arbitrary message
#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Msg {
    /// Send coins
    #[serde(rename = "cosmos-sdk/MsgSend")]
    SendMsg(SendMsg),
    /// Terra oracle vote
    #[serde(rename = "oracle/MsgExchangeRateVote")]
    MsgExchangeRateVote(MsgExchangeRateVote),
    /// Terra Oracle prevote
    #[serde(rename = "oracle/MsgExchangeRatePrevote")]
    MsgExchangeRatePrevote(MsgExchangeRatePrevote),
    /// Terra Oracle delegate voting authority
    #[serde(rename = "oracle/MsgDelegateFeedConsent")]
    MsgDelegateFeedConsent(MsgDelegateFeedConsent),
    /// Test message
    #[serde(rename = "deep_space/Test")]
    Test(String),
}

impl Msg {
    /// Transform message to canonical signable form
    pub fn to_sign_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(to_canonical_json(self)?)
    }
}

#[cfg(test)]
mod tests {
    use super::{Address, TerraAddress, TerraValidatorOperatorAddress, Msg, MsgExchangeRateVote};
    use rust_decimal::Decimal;
    use serde_json::{from_str, json, to_string, Value};
    #[test]
    fn test_serialize_msg() {
        let msg: Msg = Msg::Test("TestMsg1".to_string());
        let s = to_string(&msg).expect("Unable to serialize");
        let v: Value = from_str(&s).expect("Unable to deserialize");
        assert_eq!(v, json!({"type": "deep_space/Test", "value": "TestMsg1"}));
    }
    #[test]
    fn test_has_prevote_msg() {
        let vote: Msg = Msg::MsgExchangeRateVote(MsgExchangeRateVote {
            exchange_rate: Decimal::new(-100, 2),
            denom: "test".to_string(),
            salt: "hello_world".to_string(),
            feeder: TerraAddress{0:Address::from_bech32("terra1grgelyng2v6v3t8z87wu3sxgt9m5s03x259evd").unwrap()},
            validator: TerraValidatorOperatorAddress{0:Address::from_bech32("terravaloper1grgelyng2v6v3t8z87wu3sxgt9m5s03x2mfyu7")
                .unwrap()},
        });

        let mut hash_str: String = "".to_string();
        if let Msg::MsgExchangeRateVote(x) = vote {
            hash_str = x.generate_vote_hash();
        }
        assert_ne!(hash_str, "");
    }
}
