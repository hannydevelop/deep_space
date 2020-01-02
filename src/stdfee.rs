//! Types for Cosmos SDK fees
//!

use crate::coin::Coin;
use num256::Uint256;
use serde::Serialize;

/// StdFee includes the amount of coins paid in fees and the maximum
/// gas to be used by the transaction. The ratio yields an effective "gasprice",
/// which must be above some miminum to be accepted into the mempool.
#[derive(Serialize, Default, Debug, Clone)]
pub struct StdFee {
    /// Fee to be paid
    pub amount: Option<Vec<Coin>>,
    /// Gas requested for transaction
    pub gas: Uint256,
}
