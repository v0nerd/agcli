//! Staking extrinsics — add, remove, move, swap, limit orders.
//!
//! Maps to subtensor pallet calls:
//! - `add_stake(hotkey, netuid, amount)` — call_index 2
//! - `remove_stake(hotkey, netuid, amount)` — call_index 3
//! - `unstake_all(hotkey)` — call_index ~71
//! - `unstake_all_alpha(hotkey)` — call_index ~72
//! - `move_stake(hotkey, origin_netuid, dest_netuid, alpha_amount)` — call_index ~73
//! - `swap_stake(from_hotkey, to_hotkey, netuid, alpha_amount)` — call_index ~74
//! - `transfer_stake(dest_coldkey, hotkey, from_netuid, to_netuid, alpha_amount)` — call_index ~75
//! - `add_stake_limit(hotkey, netuid, amount, limit_price, allow_partial)` — call_index ~76
//! - `remove_stake_limit(hotkey, netuid, amount, limit_price, allow_partial)` — call_index ~77
//! - `recycle_alpha(hotkey, netuid, amount)` — call_index ~78
//! - `burn_alpha(hotkey, netuid, amount)` — call_index ~79
//! - `claim_root(hotkey, netuid)` — call_index ~80
//! - `set_coldkey_auto_stake_hotkey(hotkey, enable)` — call_index ~81

use crate::types::{Balance, NetUid};

/// Parameters for adding stake.
#[derive(Debug, Clone)]
pub struct AddStakeParams {
    pub hotkey_ss58: String,
    pub netuid: NetUid,
    pub amount: Balance,
}

/// Parameters for removing stake.
#[derive(Debug, Clone)]
pub struct RemoveStakeParams {
    pub hotkey_ss58: String,
    pub netuid: NetUid,
    pub amount: u64, // in alpha
}

/// Parameters for moving stake between subnets.
#[derive(Debug, Clone)]
pub struct MoveStakeParams {
    pub hotkey_ss58: String,
    pub origin_netuid: NetUid,
    pub dest_netuid: NetUid,
    pub alpha_amount: u64,
}

/// Parameters for swapping stake between hotkeys.
#[derive(Debug, Clone)]
pub struct SwapStakeParams {
    pub from_hotkey_ss58: String,
    pub to_hotkey_ss58: String,
    pub netuid: NetUid,
    pub alpha_amount: u64,
}

/// Parameters for transferring stake to another coldkey.
#[derive(Debug, Clone)]
pub struct TransferStakeParams {
    pub dest_coldkey_ss58: String,
    pub hotkey_ss58: String,
    pub from_netuid: NetUid,
    pub to_netuid: NetUid,
    pub alpha_amount: u64,
}

/// Parameters for limit-order stake operations.
#[derive(Debug, Clone)]
pub struct StakeLimitParams {
    pub hotkey_ss58: String,
    pub netuid: NetUid,
    pub amount: u64,
    pub limit_price: u64,
    pub allow_partial: bool,
}
