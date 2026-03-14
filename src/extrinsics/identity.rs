//! Identity extrinsics.
//!
//! Implemented on `Client` in `chain/mod.rs`:
//! - `get_identity(ss58)` — query Registry pallet IdentityOf storage
//! - `get_subnet_identity(netuid)` — query SubtensorModule SubnetIdentitiesV3
//! - `set_subnet_identity(netuid, identity)` — set SubtensorModule identity
