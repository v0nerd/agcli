//! Subnet management extrinsics.
//!
//! Implemented on `Client` in `chain/mod.rs`:
//! - `serve_axon(netuid, version, ip, port, ...)` — serve miner endpoint
//! - `set_childkey_take(hotkey, netuid, take)` — set childkey take
//! - `set_children(hotkey, netuid, children)` — set child hotkeys
//! - `dissolve_network(netuid)` — dissolve a subnet
