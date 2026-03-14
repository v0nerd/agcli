//! Weight-setting extrinsics.
//!
//! Maps to subtensor pallet calls:
//! - `set_weights(netuid, dests, weights, version_key)` — call_index 0
//! - `batch_set_weights(netuids, weights, version_keys)` — call_index 80
//! - `commit_weights(netuid, commit_hash)` — call_index 96
//! - `reveal_weights(netuid, uids, values, salt, version_key)` — call_index 97
//! - `batch_reveal_weights(...)` — call_index 98
//! - `batch_commit_weights(netuids, commit_hashes)` — call_index 100
//! - `commit_timelocked_weights(netuid, commit, reveal_round)` — TLE-based commit

use crate::types::NetUid;

/// Parameters for setting weights.
#[derive(Debug, Clone)]
pub struct SetWeightsParams {
    pub netuid: NetUid,
    pub uids: Vec<u16>,
    pub weights: Vec<u16>,
    pub version_key: u64,
}

/// Parameters for batch weight setting.
#[derive(Debug, Clone)]
pub struct BatchSetWeightsParams {
    pub netuids: Vec<NetUid>,
    pub weights: Vec<Vec<(u16, u16)>>,
    pub version_keys: Vec<u64>,
}

/// Parameters for committing weights.
#[derive(Debug, Clone)]
pub struct CommitWeightsParams {
    pub netuid: NetUid,
    pub commit_hash: [u8; 32],
}

/// Parameters for revealing weights.
#[derive(Debug, Clone)]
pub struct RevealWeightsParams {
    pub netuid: NetUid,
    pub uids: Vec<u16>,
    pub values: Vec<u16>,
    pub salt: Vec<u16>,
    pub version_key: u64,
}

/// Compute the commit hash for weight commit-reveal.
pub fn compute_weight_commit_hash(
    uids: &[u16],
    values: &[u16],
    salt: &[u16],
    version_key: u64,
) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    for uid in uids {
        hasher.update(uid.to_le_bytes());
    }
    for val in values {
        hasher.update(val.to_le_bytes());
    }
    for s in salt {
        hasher.update(s.to_le_bytes());
    }
    hasher.update(version_key.to_le_bytes());
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commit_hash_deterministic() {
        let h1 = compute_weight_commit_hash(&[1, 2], &[100, 200], &[42], 1);
        let h2 = compute_weight_commit_hash(&[1, 2], &[100, 200], &[42], 1);
        assert_eq!(h1, h2);
    }

    #[test]
    fn commit_hash_changes_with_salt() {
        let h1 = compute_weight_commit_hash(&[1, 2], &[100, 200], &[42], 1);
        let h2 = compute_weight_commit_hash(&[1, 2], &[100, 200], &[99], 1);
        assert_ne!(h1, h2);
    }
}
