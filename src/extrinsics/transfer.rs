//! Transfer extrinsics.
//!
//! Uses `Balances::transfer_allow_death` for TAO transfers between accounts.

use crate::types::Balance;

/// Parameters for a TAO transfer.
#[derive(Debug, Clone)]
pub struct TransferParams {
    pub dest_ss58: String,
    pub amount: Balance,
}
