//! Key swap extrinsics.
//!
//! - `swap_hotkey(old_hotkey, new_hotkey)` — swap hotkey
//! - `swap_coldkey(old_coldkey, new_coldkey)` — swap coldkey (deprecated, use schedule)
//! - `schedule_swap_coldkey(new_coldkey, current_block, work)` — schedule coldkey swap
//! - `announce_coldkey_swap(new_coldkey, signature)` — announce swap
//! - `swap_coldkey_announced()` — execute announced swap
//! - `dispute_coldkey_swap()` — dispute a swap
