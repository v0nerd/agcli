//! Staking extrinsics — add, remove, move, swap, limit orders.
//!
//! All staking operations are implemented directly on `Client` in `chain/mod.rs`
//! using the subxt-generated `api::tx().subtensor_module().*` calls.
//!
//! Supported operations:
//! - `add_stake(hotkey, netuid, amount)`
//! - `remove_stake(hotkey, netuid, amount)`
//! - `unstake_all(hotkey)`
//! - `move_stake(origin_hotkey, dest_hotkey, origin_netuid, dest_netuid, alpha_amount)`
//! - `swap_stake(hotkey, origin_netuid, dest_netuid, alpha_amount)`
//! - `transfer_stake(dest_coldkey, hotkey, from_netuid, to_netuid, alpha_amount)`
//! - `add_stake_limit(hotkey, netuid, amount, limit_price, allow_partial)`
//! - `remove_stake_limit(hotkey, netuid, amount, limit_price, allow_partial)`
//! - `recycle_alpha(hotkey, netuid, amount)`
//! - `claim_root(subnets)`
//! - `set_childkey_take(hotkey, netuid, take)`
//! - `set_children(hotkey, netuid, children)`
