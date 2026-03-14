//! Integration test: connect to finney and query real chain data.
//! Run with: cargo test --test chain_test -- --nocapture
//!
//! All assertions run sequentially within a single tokio runtime,
//! sharing one WebSocket connection to avoid rate-limiting.

use agcli::chain::Client;
use agcli::types::NetUid;

const FINNEY: &str = "wss://entrypoint-finney.opentensor.ai:443";

#[tokio::test]
async fn chain_queries() {
    let client = Client::connect(FINNEY).await.expect("connect to finney");

    // ── block number ──
    let block = client.get_block_number().await.expect("block number");
    assert!(
        block > 1_000_000,
        "finney should be past block 1M, got {block}"
    );
    println!("[ok] block number: {block}");

    // ── total stake ──
    let stake = client.get_total_stake().await.expect("total stake");
    assert!(stake.rao() > 0, "total stake should be nonzero");
    println!("[ok] total stake: {stake}");

    // ── total networks ──
    let n = client.get_total_networks().await.expect("total networks");
    assert!(n > 50, "finney should have >50 subnets, got {n}");
    println!("[ok] total networks: {n}");

    // ── balance ──
    let balance = client
        .get_balance_ss58("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
        .await
        .expect("balance");
    println!("[ok] Alice balance: {balance}");

    // ── all subnets ──
    let subnets = client.get_all_subnets().await.expect("subnets");
    assert!(!subnets.is_empty(), "should have subnets");
    println!("[ok] {} subnets", subnets.len());
    for s in subnets.iter().take(3) {
        println!("     {} n={} tempo={}", s.name, s.n, s.tempo);
    }

    // ── neurons lite ──
    let neurons = client.get_neurons_lite(NetUid(1)).await.expect("neurons");
    assert!(!neurons.is_empty(), "SN1 should have neurons");
    let first = &neurons[0];
    println!(
        "[ok] SN1 neurons: {} (uid0 hotkey={} stake={})",
        neurons.len(),
        &first.hotkey[..8],
        first.stake
    );

    // ── delegates ──
    let delegates = client.get_delegates().await.expect("delegates");
    assert!(!delegates.is_empty(), "should have delegates");
    println!("[ok] {} delegates", delegates.len());

    // ── stake for coldkey ──
    let stakes = client
        .get_stake_for_coldkey("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
        .await
        .expect("stakes");
    println!("[ok] Alice stakes: {}", stakes.len());

    // ── all dynamic info ──
    let dynamic = client.get_all_dynamic_info().await.expect("dynamic info");
    assert!(!dynamic.is_empty(), "should have dynamic info");
    println!("[ok] {} dynamic subnet infos", dynamic.len());
    for d in dynamic.iter().take(3) {
        println!("     SN{} \"{}\" price={:.6}", d.netuid, d.name, d.price);
    }

    // ── single dynamic info ──
    let d = client
        .get_dynamic_info(NetUid(1))
        .await
        .expect("dynamic info")
        .expect("SN1 should have dynamic info");
    assert!(d.price > 0.0, "SN1 price should be positive");
    println!(
        "[ok] SN1 dynamic: \"{}\" symbol={} price={:.6}",
        d.name, d.symbol, d.price
    );
}
