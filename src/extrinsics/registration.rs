//! Registration extrinsics — register neurons, subnets.
//!
//! Maps to subtensor pallet calls:
//! - `register(netuid, block_hash, difficulty, nonce, work, hotkey, coldkey)` — POW registration
//! - `burned_register(netuid, hotkey)` — burn TAO to register
//! - `root_register(hotkey)` — register on root network
//! - `register_network(hotkey)` — create a new subnet
//! - `register_network_with_identity(hotkey, identity)` — create subnet with identity
//! - `dissolve_network(netuid)` — dissolve a subnet

use crate::types::NetUid;

/// Parameters for POW registration.
#[derive(Debug, Clone)]
pub struct PowRegisterParams {
    pub netuid: NetUid,
    pub block_hash: [u8; 32],
    pub difficulty: u64,
    pub nonce: u64,
    pub work: [u8; 32],
    pub hotkey_ss58: String,
    pub coldkey_ss58: String,
}

/// Parameters for burn registration.
#[derive(Debug, Clone)]
pub struct BurnRegisterParams {
    pub netuid: NetUid,
    pub hotkey_ss58: String,
}

/// Parameters for registering a new subnet.
#[derive(Debug, Clone)]
pub struct RegisterNetworkParams {
    pub hotkey_ss58: String,
    pub identity: Option<SubnetIdentityParams>,
}

/// Subnet identity for registration.
#[derive(Debug, Clone)]
pub struct SubnetIdentityParams {
    pub name: String,
    pub github_repo: String,
    pub contact: String,
    pub url: String,
    pub discord: String,
    pub description: String,
}
