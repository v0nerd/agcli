//! Weight-setting extrinsics and commit-reveal hashing.
//!
//! Extrinsics implemented on `Client` in `chain/mod.rs`:
//! - `set_weights(netuid, uids, weights, version_key)`
//! - `commit_weights(netuid, commit_hash)`
//! - `reveal_weights(netuid, uids, values, salt, version_key)`

/// Compute the commit hash for weight commit-reveal (SHA-256 based).
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
