//! Registration extrinsics — register neurons, subnets.
//!
//! Implemented on `Client` in `chain/mod.rs`:
//! - `burned_register(netuid, hotkey)` — burn TAO to register
//! - `pow_register(netuid, hotkey, block_number, nonce, work)` — POW registration
//! - `root_register(hotkey)` — register on root network
//! - `register_network(hotkey)` — create a new subnet
