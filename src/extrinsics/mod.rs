//! Extrinsic construction and submission.
//!
//! Each submodule corresponds to a category of chain operations.
//! All extrinsics are built as SCALE-encoded call data and submitted
//! via `author_submitExtrinsic` RPC.

pub mod staking;
pub mod transfer;
pub mod registration;
pub mod weights;
pub mod subnet;
pub mod identity;
pub mod swap;
