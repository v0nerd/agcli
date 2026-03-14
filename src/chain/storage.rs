//! Chain storage queries via RPC.

use anyhow::Result;
use sp_core::sr25519;
use crate::types::Balance;
use super::connection::rpc_call;

/// Query the balance of an account.
pub async fn get_balance(rpc_url: &str, account: &sr25519::Public) -> Result<Balance> {
    let account_hex = format!("0x{}", hex::encode(account.0));
    let result = rpc_call(
        rpc_url,
        "system_account",
        vec![serde_json::Value::String(account_hex)],
    )
    .await;

    match result {
        Ok(val) => {
            // Parse the account info to extract free balance
            if let Some(data) = val.get("data") {
                if let Some(free) = data.get("free").and_then(|v| v.as_str()) {
                    let rao = u64::from_str_radix(free.trim_start_matches("0x"), 16)
                        .unwrap_or(0);
                    return Ok(Balance::from_rao(rao));
                }
            }
            Ok(Balance::ZERO)
        }
        Err(_) => Ok(Balance::ZERO),
    }
}

/// Get current block number.
pub async fn get_block_number(rpc_url: &str) -> Result<u64> {
    let header = rpc_call(rpc_url, "chain_getHeader", vec![]).await?;
    let number_hex = header
        .get("number")
        .and_then(|v| v.as_str())
        .unwrap_or("0x0");
    let number = u64::from_str_radix(number_hex.trim_start_matches("0x"), 16)?;
    Ok(number)
}

/// Query a raw storage key.
pub async fn get_storage(rpc_url: &str, storage_key: &str) -> Result<Option<String>> {
    let result = rpc_call(
        rpc_url,
        "state_getStorage",
        vec![serde_json::Value::String(storage_key.to_string())],
    )
    .await?;

    Ok(result.as_str().map(|s| s.to_string()))
}

/// Query storage at a specific block hash.
pub async fn get_storage_at(
    rpc_url: &str,
    storage_key: &str,
    block_hash: &str,
) -> Result<Option<String>> {
    let result = rpc_call(
        rpc_url,
        "state_getStorage",
        vec![
            serde_json::Value::String(storage_key.to_string()),
            serde_json::Value::String(block_hash.to_string()),
        ],
    )
    .await?;

    Ok(result.as_str().map(|s| s.to_string()))
}
