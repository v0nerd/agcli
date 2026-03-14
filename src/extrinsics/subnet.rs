//! Subnet management extrinsics.
//!
//! - `serve_axon(netuid, version, ip, port, ...)` — serve miner endpoint
//! - `serve_prometheus(netuid, version, ip, port, ...)` — serve prometheus
//! - `set_childkey_take(hotkey, netuid, take)` — set childkey take
//! - `set_children(hotkey, netuid, children)` — set child hotkeys

use crate::types::NetUid;

/// Parameters for serving an axon.
#[derive(Debug, Clone)]
pub struct ServeAxonParams {
    pub netuid: NetUid,
    pub version: u32,
    pub ip: u128,
    pub port: u16,
    pub ip_type: u8,
    pub protocol: u8,
    pub placeholder1: u8,
    pub placeholder2: u8,
}

/// Parameters for setting children.
#[derive(Debug, Clone)]
pub struct SetChildrenParams {
    pub hotkey_ss58: String,
    pub netuid: NetUid,
    /// (proportion, child_hotkey_ss58)
    pub children: Vec<(u64, String)>,
}

/// Parameters for setting childkey take.
#[derive(Debug, Clone)]
pub struct SetChildkeyTakeParams {
    pub hotkey_ss58: String,
    pub netuid: NetUid,
    pub take: u16,
}
