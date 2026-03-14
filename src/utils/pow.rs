//! Proof-of-work solver for POW-based subnet registration.
//!
//! Subtensor allows miners to register by finding a nonce such that
//! hash(block_hash || hotkey || nonce) meets a difficulty target.

use blake2::{Blake2b512, Digest};

/// Attempt to solve POW for registration.
/// Returns (nonce, seal_hash) if found within max_attempts.
pub fn solve_pow(
    block_hash: &[u8; 32],
    hotkey_bytes: &[u8; 32],
    difficulty: u64,
    max_attempts: u64,
) -> Option<(u64, [u8; 32])> {
    solve_pow_range(block_hash, hotkey_bytes, difficulty, 0, max_attempts)
}

/// Solve POW in a specific nonce range (for multi-threaded solving).
pub fn solve_pow_range(
    block_hash: &[u8; 32],
    hotkey_bytes: &[u8; 32],
    difficulty: u64,
    start_nonce: u64,
    count: u64,
) -> Option<(u64, [u8; 32])> {
    let target = u64::MAX / difficulty;

    for nonce in start_nonce..start_nonce.saturating_add(count) {
        let hash = compute_pow_hash(block_hash, hotkey_bytes, nonce);
        let score = u64::from_le_bytes(hash[..8].try_into().unwrap());
        if score <= target {
            return Some((nonce, hash));
        }
    }
    None
}

/// Compute the POW hash.
fn compute_pow_hash(block_hash: &[u8; 32], hotkey: &[u8; 32], nonce: u64) -> [u8; 32] {
    let mut hasher = Blake2b512::new();
    hasher.update(block_hash);
    hasher.update(hotkey);
    hasher.update(nonce.to_le_bytes());
    let result = hasher.finalize();

    // Take first 32 bytes
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result[..32]);
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pow_hash_deterministic() {
        let block = [0u8; 32];
        let hotkey = [1u8; 32];
        let h1 = compute_pow_hash(&block, &hotkey, 42);
        let h2 = compute_pow_hash(&block, &hotkey, 42);
        assert_eq!(h1, h2);
    }

    #[test]
    fn pow_hash_changes_with_nonce() {
        let block = [0u8; 32];
        let hotkey = [1u8; 32];
        let h1 = compute_pow_hash(&block, &hotkey, 0);
        let h2 = compute_pow_hash(&block, &hotkey, 1);
        assert_ne!(h1, h2);
    }
}
