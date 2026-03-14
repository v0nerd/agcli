//! Portfolio queries — aggregate all stakes, balances, and subnet positions.

use crate::chain::Client;
use crate::types::Balance;
use anyhow::Result;
use serde::Serialize;

/// A user's complete portfolio across all subnets.
#[derive(Debug, Serialize)]
pub struct Portfolio {
    pub coldkey_ss58: String,
    pub free_balance: Balance,
    pub total_staked: Balance,
    pub positions: Vec<SubnetPosition>,
}

/// Stake position on a single subnet.
#[derive(Debug, Serialize)]
pub struct SubnetPosition {
    pub netuid: u16,
    pub subnet_name: String,
    pub hotkey_ss58: String,
    pub alpha_stake: u64,
    pub tao_equivalent: Balance,
    pub price: f64,
}

/// Fetch the full portfolio for a coldkey.
pub async fn fetch_portfolio(client: &Client, coldkey_ss58: &str) -> Result<Portfolio> {
    let balance = client.get_balance_ss58(coldkey_ss58).await?;
    let stakes = client.get_stake_for_coldkey(coldkey_ss58).await?;

    let positions: Vec<SubnetPosition> = stakes
        .iter()
        .map(|s| SubnetPosition {
            netuid: s.netuid.0,
            subnet_name: String::new(), // TODO: resolve
            hotkey_ss58: s.hotkey.clone(),
            alpha_stake: s.alpha_stake.raw(),
            tao_equivalent: s.stake,
            price: 0.0, // TODO: fetch dynamic price
        })
        .collect();

    let total_staked = positions.iter().fold(Balance::ZERO, |acc, p| acc + p.tao_equivalent);

    Ok(Portfolio {
        coldkey_ss58: coldkey_ss58.to_string(),
        free_balance: balance,
        total_staked,
        positions,
    })
}
