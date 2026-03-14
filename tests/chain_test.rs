//! Integration test: connect to finney and query real chain data.
//! Run with: cargo test --test chain_test -- --nocapture
//!
//! Uses a shared client (single WebSocket) to avoid rate limiting on the Finney endpoint.

use agcli::chain::Client;
use agcli::types::NetUid;
use std::sync::Arc;
use tokio::sync::OnceCell;

const FINNEY: &str = "wss://entrypoint-finney.opentensor.ai:443";

/// Shared client across all tests in this file — single WebSocket connection.
static CLIENT: OnceCell<Arc<Client>> = OnceCell::const_new();

async fn client() -> &'static Arc<Client> {
    CLIENT
        .get_or_init(|| async {
            Arc::new(Client::connect(FINNEY).await.expect("connect to finney"))
        })
        .await
}

#[tokio::test]
async fn test_connect_and_block_number() {
    let client = client().await;
    let block = client.get_block_number().await.expect("block number");
    assert!(
        block > 1_000_000,
        "finney should be past block 1M, got {block}"
    );
    println!("Current block: {block}");
}

#[tokio::test]
async fn test_total_stake() {
    let client = client().await;
    let stake = client.get_total_stake().await.expect("total stake");
    println!("Total stake: {stake}");
    assert!(stake.rao() > 0, "total stake should be nonzero");
}

#[tokio::test]
async fn test_total_networks() {
    let client = client().await;
    let n = client.get_total_networks().await.expect("total networks");
    println!("Total networks: {n}");
    assert!(n > 50, "finney should have >50 subnets, got {n}");
}

#[tokio::test]
async fn test_get_balance() {
    let client = client().await;
    // Query a known address (opentensor foundation)
    let balance = client
        .get_balance_ss58("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
        .await
        .expect("balance");
    println!("Alice balance: {balance}");
}

#[tokio::test]
async fn test_get_all_subnets() {
    let client = client().await;
    let subnets = client.get_all_subnets().await.expect("subnets");
    println!("Got {} subnets", subnets.len());
    assert!(!subnets.is_empty(), "should have subnets");
    for s in subnets.iter().take(5) {
        println!(
            "  {} n={} tempo={} owner={}",
            s.name,
            s.n,
            s.tempo,
            &s.owner[..8]
        );
    }
}

#[tokio::test]
async fn test_get_neurons_lite() {
    let client = client().await;
    let neurons = client.get_neurons_lite(NetUid(1)).await.expect("neurons");
    println!("SN1 neurons: {}", neurons.len());
    assert!(!neurons.is_empty(), "SN1 should have neurons");
    let first = &neurons[0];
    println!(
        "  UID={} hotkey={} stake={} rank={:.4}",
        first.uid,
        &first.hotkey[..8],
        first.stake,
        first.rank
    );
}

#[tokio::test]
async fn test_get_delegates() {
    let client = client().await;
    let delegates = client.get_delegates().await.expect("delegates");
    println!("Got {} delegates", delegates.len());
    assert!(!delegates.is_empty(), "should have delegates");
}

#[tokio::test]
async fn test_get_stake_for_coldkey() {
    let client = client().await;
    let stakes = client
        .get_stake_for_coldkey("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
        .await
        .expect("stakes");
    println!("Stakes for Alice: {}", stakes.len());
}

#[tokio::test]
async fn test_get_all_dynamic_info() {
    let client = client().await;
    let dynamic = client.get_all_dynamic_info().await.expect("dynamic info");
    println!("Got {} dynamic subnet infos", dynamic.len());
    assert!(!dynamic.is_empty(), "should have dynamic info");
    for d in dynamic.iter().take(5) {
        println!(
            "  SN{} \"{}\" price={:.6} tao_in={} alpha_in={} alpha_out={}",
            d.netuid, d.name, d.price, d.tao_in, d.alpha_in, d.alpha_out
        );
    }
}

#[tokio::test]
async fn test_get_dynamic_info_single() {
    let client = client().await;
    let dynamic = client
        .get_dynamic_info(NetUid(1))
        .await
        .expect("dynamic info");
    assert!(dynamic.is_some(), "SN1 should have dynamic info");
    let d = dynamic.unwrap();
    println!(
        "SN1: \"{}\" symbol={} price={:.6}",
        d.name, d.symbol, d.price
    );
    assert!(d.price > 0.0, "SN1 price should be positive");
}
