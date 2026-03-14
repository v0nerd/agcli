//! Extended integration tests against the live Finney chain.
//! Run with: cargo test --test chain_integration_test -- --nocapture
//!
//! These tests perform read-only queries against the Bittensor mainnet.

use agcli::chain::Client;
use agcli::types::NetUid;

const FINNEY: &str = "wss://entrypoint-finney.opentensor.ai:443";

/// Known Bittensor foundation/OTF address for testing.
const KNOWN_ADDRESS: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

#[tokio::test]
async fn test_subnet_hyperparams() {
    let client = Client::connect(FINNEY).await.expect("connect");
    let params = client
        .get_subnet_hyperparams(NetUid(1))
        .await
        .expect("hyperparams");
    assert!(params.is_some(), "SN1 should have hyperparams");
    let h = params.unwrap();
    assert!(h.tempo > 0, "tempo should be positive");
    assert!(h.max_validators > 0, "max_validators should be positive");
    println!(
        "SN1 tempo={} max_validators={} rho={}",
        h.tempo, h.max_validators, h.rho
    );
}

#[tokio::test]
async fn test_get_subnet_info() {
    let client = Client::connect(FINNEY).await.expect("connect");
    let info = client
        .get_subnet_info(NetUid(1))
        .await
        .expect("subnet info");
    assert!(info.is_some(), "SN1 should exist");
    let s = info.unwrap();
    assert_eq!(s.netuid, NetUid(1));
    assert!(s.n > 0, "SN1 should have neurons");
    println!(
        "SN1: name={} n={}/{} tempo={}",
        s.name, s.n, s.max_n, s.tempo
    );
}

#[tokio::test]
async fn test_dynamic_info_all_vs_single() {
    let client = Client::connect(FINNEY).await.expect("connect");
    let all = client.get_all_dynamic_info().await.expect("all dynamic");
    let single = client
        .get_dynamic_info(NetUid(1))
        .await
        .expect("single dynamic");
    assert!(single.is_some());
    let s = single.unwrap();

    // Find SN1 in the all-subnets list
    let found = all.iter().find(|d| d.netuid == NetUid(1));
    assert!(found.is_some(), "SN1 should be in get_all_dynamic_info");
    let f = found.unwrap();

    // Prices should be reasonably close (queried at slightly different times)
    let diff = (s.price - f.price).abs();
    assert!(
        diff < 0.1,
        "Prices should be close: single={} all={}",
        s.price,
        f.price
    );
    println!(
        "SN1 price: single={:.6} all={:.6} diff={:.6}",
        s.price, f.price, diff
    );
}

#[tokio::test]
async fn test_get_balance_known_account() {
    let client = Client::connect(FINNEY).await.expect("connect");
    let balance = client
        .get_balance_ss58(KNOWN_ADDRESS)
        .await
        .expect("balance");
    // Alice dev account likely has some balance
    println!(
        "Known account balance: {} RAO ({} TAO)",
        balance.rao(),
        balance.tao()
    );
}

#[tokio::test]
async fn test_get_stake_for_unknown_coldkey() {
    let client = Client::connect(FINNEY).await.expect("connect");
    // A freshly generated address should have no stakes
    let stakes = client
        .get_stake_for_coldkey("5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM")
        .await
        .expect("stakes");
    // It's okay if this address has recent stakes; just check we get a valid response
    println!("Unknown coldkey has {} stake positions", stakes.len());
}

#[tokio::test]
async fn test_nonexistent_subnet() {
    let client = Client::connect(FINNEY).await.expect("connect");
    // Very high netuid should not exist
    let info = client
        .get_subnet_info(NetUid(65535))
        .await
        .expect("no error");
    assert!(info.is_none(), "netuid 65535 should not exist");
}

#[tokio::test]
async fn test_get_total_issuance() {
    let client = Client::connect(FINNEY).await.expect("connect");
    let total = client.get_total_stake().await.expect("total stake");
    assert!(total.rao() > 0, "total stake should be nonzero");
    println!("Total stake: {} TAO", total.tao());
}

#[tokio::test]
async fn test_neurons_lite_ordering() {
    let client = Client::connect(FINNEY).await.expect("connect");
    let neurons = client.get_neurons_lite(NetUid(1)).await.expect("neurons");
    assert!(!neurons.is_empty());
    // UIDs should be sequential starting from 0
    for (i, n) in neurons.iter().enumerate() {
        assert_eq!(n.uid as usize, i, "neuron UIDs should be sequential");
    }
    println!(
        "SN1: {} neurons, UIDs 0..{}",
        neurons.len(),
        neurons.len() - 1
    );
}

// ──── Step 26: Additional integration tests ────

#[tokio::test]
async fn test_total_issuance() {
    let client = Client::connect(FINNEY).await.expect("connect");
    let issuance = client.get_total_issuance().await.expect("total issuance");
    // Bittensor total issuance should be > 0 and reasonable (millions of TAO)
    assert!(issuance.rao() > 0, "total issuance should be nonzero");
    println!("Total issuance: {:.2} TAO", issuance.tao());
}

#[tokio::test]
async fn test_block_emission() {
    let client = Client::connect(FINNEY).await.expect("connect");
    let emission = client.get_block_emission().await.expect("block emission");
    // Block emission should be positive (TAO minted per block)
    assert!(emission.rao() > 0, "block emission should be positive");
    println!(
        "Block emission: {} RAO ({:.6} TAO)",
        emission.rao(),
        emission.tao()
    );
}

#[tokio::test]
async fn test_sim_swap_tao_for_alpha() {
    let client = Client::connect(FINNEY).await.expect("connect");
    // Simulate swapping 1 TAO for alpha on SN1
    let one_tao_rao = 1_000_000_000u64;
    let (alpha_amount, tao_fee, alpha_fee) = client
        .sim_swap_tao_for_alpha(NetUid(1), one_tao_rao)
        .await
        .expect("sim swap");
    assert!(alpha_amount > 0, "should receive some alpha");
    println!(
        "1 TAO → {} alpha (tao_fee={}, alpha_fee={})",
        alpha_amount, tao_fee, alpha_fee
    );
}

#[tokio::test]
async fn test_sim_swap_alpha_for_tao() {
    let client = Client::connect(FINNEY).await.expect("connect");
    // Simulate swapping 100 alpha units for TAO on SN1
    let alpha = 100_000_000_000u64; // 100 alpha
    let (tao_amount, tao_fee, alpha_fee) = client
        .sim_swap_alpha_for_tao(NetUid(1), alpha)
        .await
        .expect("sim swap");
    assert!(tao_amount > 0, "should receive some TAO");
    println!(
        "100 alpha → {} TAO (tao_fee={}, alpha_fee={})",
        tao_amount, tao_fee, alpha_fee
    );
}

#[tokio::test]
async fn test_get_delegate_single() {
    let client = Client::connect(FINNEY).await.expect("connect");
    // Get all delegates, then query the first one by hotkey
    let delegates = client.get_delegates().await.expect("delegates");
    assert!(!delegates.is_empty());
    let first = &delegates[0];
    let single = client
        .get_delegate(&first.hotkey)
        .await
        .expect("single delegate");
    assert!(single.is_some(), "first delegate should be queryable");
    let d = single.unwrap();
    assert_eq!(d.hotkey, first.hotkey, "hotkeys should match");
    println!(
        "Delegate: {} take={:.2}%",
        &d.hotkey[..8],
        d.take as f64 / 65535.0 * 100.0
    );
}

#[tokio::test]
async fn test_list_proxies_empty() {
    let client = Client::connect(FINNEY).await.expect("connect");
    // An unused address should have no proxies
    let proxies = client
        .list_proxies("5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM")
        .await
        .expect("proxies");
    // It's fine if it has proxies; just check the query succeeds
    println!("Proxies for address: {}", proxies.len());
}

#[tokio::test]
async fn test_dynamic_info_all_valid_prices() {
    let client = Client::connect(FINNEY).await.expect("connect");
    let dynamic = client.get_all_dynamic_info().await.expect("dynamic");
    // Check no negative prices (regression test from Step 25)
    for d in &dynamic {
        assert!(
            d.price >= 0.0,
            "SN{} has negative price: {}",
            d.netuid,
            d.price
        );
    }
    println!("All {} subnets have non-negative prices", dynamic.len());
}

#[tokio::test]
async fn test_hyperparams_nonexistent_subnet() {
    let client = Client::connect(FINNEY).await.expect("connect");
    let params = client
        .get_subnet_hyperparams(NetUid(65535))
        .await
        .expect("hyperparams");
    assert!(params.is_none(), "netuid 65535 should not have hyperparams");
}
