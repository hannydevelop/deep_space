//! Cosmos SDK account Addresses
//!

use failure::{ensure, Error};
use serde::Serialize;
use serde::Serializer;
use std::fmt::{self, Display};
use subtle_encoding::bech32::{self};

/// A Cosmos Hub address that's derived from a given PublicKey
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub struct Address([u8; 20]);

/// A Cosmos Hub Validator Operator address that's derived from a given PublicKey
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub struct CosmosValidatorOperatorAddress(pub Address);
/// A Terra address that's derived from a given PublicKey
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub struct TerraAddress(pub Address);
/// A Terra Validator Operator address that's derived from a given PublicKey
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub struct TerraValidatorOperatorAddress(pub Address);

impl Address {
    /// Create an address from byte array.
    pub fn from_bytes(bytes: [u8; 20]) -> Address {
        Address(bytes)
    }

    /// Obtain a bech32 encoded address with a given prefix.
    ///
    /// * `hrp` - A prefix for bech32 encoding. The convention for addresses
    /// in Cosmos is `cosmos`.
    pub fn to_bech32<T: Into<String>>(&self, hrp: T) -> String {
        bech32::encode(&hrp.into(), self.0)
    }

    /// Parse a bech32 encoded address
    ///
    /// * `s` - A bech32 encoded address
    pub fn from_bech32<S: Into<String>>(s: S) -> Result<Address, Error> {
        let (_hrp, data) = bech32::decode(s.into())?;
        let mut addr = [0u8; 20];
        ensure!(data.len() == 20, "Wrong size of decoded bech32 data");
        addr.copy_from_slice(&data);
        Ok(Address(addr))
    }
}

impl Display for Address {
    fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
        write!(f, "{}", self.to_bech32("cosmos"))?;
        Ok(())
    }
}

impl Display for TerraAddress {
    fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
        write!(f, "{}", self.0.to_bech32("terra"))?;
        Ok(())
    }
}

impl Display for TerraValidatorOperatorAddress {
    fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
        write!(f, "{}", self.0.to_bech32("terravaloper"))?;
        Ok(())
    }
}

impl Display for CosmosValidatorOperatorAddress {
    fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
        write!(f, "{}", self.0.to_bech32("cosmosvaloper"))?;
        Ok(())
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize address as a string with a default prefix for addresses
        let s = self.to_bech32("cosmos");
        serializer.serialize_str(&s)
    }
}

impl Serialize for CosmosValidatorOperatorAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize address as a string with a default prefix for addresses
        let s = self.0.to_bech32("cosmosvaloper");
        serializer.serialize_str(&s)
    }
}

impl Serialize for TerraAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize address as a string with a default prefix for addresses
        let s = self.0.to_bech32("terra");
        serializer.serialize_str(&s)
    }
}

impl Serialize for TerraValidatorOperatorAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize address as a string with a default prefix for addresses
        let s = self.0.to_bech32("terra");
        serializer.serialize_str(&s)
    }
}

#[test]
fn test_bech32() {
    let address = Address::default();
    assert_eq!(
        address.to_bech32("cosmos"),
        "cosmos1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqnrql8a"
    );

    let decoded = Address::from_bech32("cosmos1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqnrql8a".to_string())
        .expect("Unable to decode");
    assert_eq!(address, decoded);
}
