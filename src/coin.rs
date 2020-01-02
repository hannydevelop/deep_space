//! Cosmos SDK Coins Struct
//!

use num256::Uint256;
use serde::Serialize;

/// Coin holds some amount of one currency
#[derive(Serialize, Debug, Default, Clone)]
pub struct Coin {
    /// Amount of Coins to use in message
    pub amount: Uint256,
    /// Denominsation of Coin to use
    pub denom: String,
}

impl Coin {
    /// Contruct a new Coin
    pub fn new(amount: Uint256, denom: String) -> Coin {
        Coin { amount, denom }
    }
}
