//! Substrate chain client — connect, query storage, submit extrinsics.

pub mod connection;
pub mod storage;

use anyhow::Result;
use sp_core::sr25519;
use crate::types::{Balance, NetUid};
use crate::types::chain_data::*;

/// High-level client for the Bittensor (subtensor) chain.
pub struct Client {
    pub(crate) url: String,
    // In a full implementation this wraps a subxt::OnlineClient.
    // For now we use RPC calls via reqwest/jsonrpsee.
    pub(crate) rpc_url: String,
}

impl Client {
    /// Connect to a subtensor node.
    pub async fn connect(url: &str) -> Result<Self> {
        // Convert wss:// to https:// for RPC fallback
        let rpc_url = url
            .replace("wss://", "https://")
            .replace("ws://", "http://");

        tracing::info!("Connecting to {}", url);
        Ok(Self {
            url: url.to_string(),
            rpc_url,
        })
    }

    /// Connect to a well-known network.
    pub async fn connect_network(network: &crate::types::Network) -> Result<Self> {
        Self::connect(network.ws_url()).await
    }

    // ──────── Balance Queries ────────

    /// Get TAO balance (free + reserved) for an account.
    pub async fn get_balance(&self, account: &sr25519::Public) -> Result<Balance> {
        storage::get_balance(&self.rpc_url, account).await
    }

    /// Get TAO balance for an SS58 address.
    pub async fn get_balance_ss58(&self, ss58: &str) -> Result<Balance> {
        let pk = crate::wallet::keypair::from_ss58(ss58)?;
        self.get_balance(&pk).await
    }

    // ──────── Stake Queries ────────

    /// Get all stakes for a coldkey.
    pub async fn get_stake_for_coldkey(&self, _coldkey_ss58: &str) -> Result<Vec<StakeInfo>> {
        // TODO: implement via RPC
        Ok(vec![])
    }

    /// Get total hotkey alpha on a subnet.
    pub async fn get_total_hotkey_alpha(
        &self,
        _hotkey_ss58: &str,
        _netuid: NetUid,
    ) -> Result<Balance> {
        Ok(Balance::ZERO)
    }

    // ──────── Subnet Queries ────────

    /// List all subnets.
    pub async fn get_all_subnets(&self) -> Result<Vec<SubnetInfo>> {
        Ok(vec![])
    }

    /// Get dynamic info for all subnets.
    pub async fn get_all_dynamic_info(&self) -> Result<Vec<DynamicInfo>> {
        Ok(vec![])
    }

    /// Get info for a specific subnet.
    pub async fn get_subnet_info(&self, _netuid: NetUid) -> Result<Option<SubnetInfo>> {
        Ok(None)
    }

    /// Get subnet hyperparameters.
    pub async fn get_subnet_hyperparams(
        &self,
        _netuid: NetUid,
    ) -> Result<Option<SubnetHyperparameters>> {
        Ok(None)
    }

    // ──────── Neuron / Metagraph Queries ────────

    /// Get the metagraph for a subnet.
    pub async fn get_metagraph(&self, _netuid: NetUid) -> Result<Metagraph> {
        anyhow::bail!("Not yet implemented — use get_neurons_lite for now")
    }

    /// Get lightweight neuron info for a subnet.
    pub async fn get_neurons_lite(&self, _netuid: NetUid) -> Result<Vec<NeuronInfoLite>> {
        Ok(vec![])
    }

    /// Get full neuron info for specific UID.
    pub async fn get_neuron(
        &self,
        _netuid: NetUid,
        _uid: u16,
    ) -> Result<Option<NeuronInfo>> {
        Ok(None)
    }

    // ──────── Delegate Queries ────────

    /// Get all delegates.
    pub async fn get_delegates(&self) -> Result<Vec<DelegateInfo>> {
        Ok(vec![])
    }

    /// Get delegate info for a specific hotkey.
    pub async fn get_delegate(&self, _hotkey_ss58: &str) -> Result<Option<DelegateInfo>> {
        Ok(None)
    }

    // ──────── Identity Queries ────────

    /// Get on-chain identity for an account.
    pub async fn get_identity(&self, _ss58: &str) -> Result<Option<ChainIdentity>> {
        Ok(None)
    }

    /// Get subnet identity.
    pub async fn get_subnet_identity(
        &self,
        _netuid: NetUid,
    ) -> Result<Option<SubnetIdentity>> {
        Ok(None)
    }

    // ──────── Network Params ────────

    /// Current block number.
    pub async fn get_block_number(&self) -> Result<u64> {
        storage::get_block_number(&self.rpc_url).await
    }

    /// Total TAO issuance.
    pub async fn get_total_issuance(&self) -> Result<Balance> {
        Ok(Balance::ZERO)
    }

    /// Total staked TAO.
    pub async fn get_total_stake(&self) -> Result<Balance> {
        Ok(Balance::ZERO)
    }

    /// Total number of subnets.
    pub async fn get_total_networks(&self) -> Result<u16> {
        Ok(0)
    }

    /// Block emission rate.
    pub async fn get_block_emission(&self) -> Result<Balance> {
        Ok(Balance::ZERO)
    }

    // ──────── Extrinsic Submission ────────

    /// Transfer TAO from coldkey to destination.
    pub async fn transfer(
        &self,
        _signer: &sr25519::Pair,
        _dest_ss58: &str,
        _amount: Balance,
    ) -> Result<String> {
        anyhow::bail!("Transfer not yet implemented")
    }

    /// Add stake to a hotkey on a subnet.
    pub async fn add_stake(
        &self,
        _signer: &sr25519::Pair,
        _hotkey_ss58: &str,
        _netuid: NetUid,
        _amount: Balance,
    ) -> Result<String> {
        anyhow::bail!("add_stake not yet implemented")
    }

    /// Remove stake from a hotkey on a subnet.
    pub async fn remove_stake(
        &self,
        _signer: &sr25519::Pair,
        _hotkey_ss58: &str,
        _netuid: NetUid,
        _amount: Balance,
    ) -> Result<String> {
        anyhow::bail!("remove_stake not yet implemented")
    }

    /// Register on a subnet via burned TAO.
    pub async fn burned_register(
        &self,
        _signer: &sr25519::Pair,
        _netuid: NetUid,
        _hotkey_ss58: &str,
    ) -> Result<String> {
        anyhow::bail!("burned_register not yet implemented")
    }

    /// Set weights on a subnet.
    pub async fn set_weights(
        &self,
        _signer: &sr25519::Pair,
        _netuid: NetUid,
        _uids: &[u16],
        _weights: &[u16],
        _version_key: u64,
    ) -> Result<String> {
        anyhow::bail!("set_weights not yet implemented")
    }

    /// Register a new subnet.
    pub async fn register_network(
        &self,
        _signer: &sr25519::Pair,
        _hotkey_ss58: &str,
    ) -> Result<String> {
        anyhow::bail!("register_network not yet implemented")
    }

    /// Move stake between subnets.
    pub async fn move_stake(
        &self,
        _signer: &sr25519::Pair,
        _hotkey_ss58: &str,
        _from_netuid: NetUid,
        _to_netuid: NetUid,
        _amount: Balance,
    ) -> Result<String> {
        anyhow::bail!("move_stake not yet implemented")
    }

    /// Swap stake between hotkeys.
    pub async fn swap_stake(
        &self,
        _signer: &sr25519::Pair,
        _from_hotkey: &str,
        _to_hotkey: &str,
        _netuid: NetUid,
        _amount: Balance,
    ) -> Result<String> {
        anyhow::bail!("swap_stake not yet implemented")
    }

    /// Set childkey take.
    pub async fn set_childkey_take(
        &self,
        _signer: &sr25519::Pair,
        _hotkey_ss58: &str,
        _netuid: NetUid,
        _take: u16,
    ) -> Result<String> {
        anyhow::bail!("set_childkey_take not yet implemented")
    }

    /// Set children for a hotkey.
    pub async fn set_children(
        &self,
        _signer: &sr25519::Pair,
        _hotkey_ss58: &str,
        _netuid: NetUid,
        _children: &[(u64, String)],
    ) -> Result<String> {
        anyhow::bail!("set_children not yet implemented")
    }

    /// Commit weights (commit-reveal scheme).
    pub async fn commit_weights(
        &self,
        _signer: &sr25519::Pair,
        _netuid: NetUid,
        _commit_hash: [u8; 32],
    ) -> Result<String> {
        anyhow::bail!("commit_weights not yet implemented")
    }

    /// Reveal weights.
    pub async fn reveal_weights(
        &self,
        _signer: &sr25519::Pair,
        _netuid: NetUid,
        _uids: &[u16],
        _values: &[u16],
        _salt: &[u16],
        _version_key: u64,
    ) -> Result<String> {
        anyhow::bail!("reveal_weights not yet implemented")
    }

    /// Serve axon metadata.
    pub async fn serve_axon(
        &self,
        _signer: &sr25519::Pair,
        _netuid: NetUid,
        _axon: &AxonInfo,
    ) -> Result<String> {
        anyhow::bail!("serve_axon not yet implemented")
    }

    /// Set on-chain identity.
    pub async fn set_identity(
        &self,
        _signer: &sr25519::Pair,
        _identity: &ChainIdentity,
    ) -> Result<String> {
        anyhow::bail!("set_identity not yet implemented")
    }

    /// Decrease delegate take.
    pub async fn decrease_take(
        &self,
        _signer: &sr25519::Pair,
        _hotkey_ss58: &str,
        _take: u16,
    ) -> Result<String> {
        anyhow::bail!("decrease_take not yet implemented")
    }

    /// Increase delegate take.
    pub async fn increase_take(
        &self,
        _signer: &sr25519::Pair,
        _hotkey_ss58: &str,
        _take: u16,
    ) -> Result<String> {
        anyhow::bail!("increase_take not yet implemented")
    }

    /// Unstake all from a hotkey.
    pub async fn unstake_all(
        &self,
        _signer: &sr25519::Pair,
        _hotkey_ss58: &str,
    ) -> Result<String> {
        anyhow::bail!("unstake_all not yet implemented")
    }

    /// Schedule coldkey swap.
    pub async fn schedule_swap_coldkey(
        &self,
        _signer: &sr25519::Pair,
        _new_coldkey_ss58: &str,
    ) -> Result<String> {
        anyhow::bail!("schedule_swap_coldkey not yet implemented")
    }

    /// Swap hotkey.
    pub async fn swap_hotkey(
        &self,
        _signer: &sr25519::Pair,
        _old_hotkey_ss58: &str,
        _new_hotkey_ss58: &str,
    ) -> Result<String> {
        anyhow::bail!("swap_hotkey not yet implemented")
    }

    /// Root register.
    pub async fn root_register(
        &self,
        _signer: &sr25519::Pair,
        _hotkey_ss58: &str,
    ) -> Result<String> {
        anyhow::bail!("root_register not yet implemented")
    }

    /// Dissolve a subnet.
    pub async fn dissolve_network(
        &self,
        _signer: &sr25519::Pair,
        _netuid: NetUid,
    ) -> Result<String> {
        anyhow::bail!("dissolve_network not yet implemented")
    }

    /// Transfer stake to another coldkey.
    pub async fn transfer_stake(
        &self,
        _signer: &sr25519::Pair,
        _dest_coldkey_ss58: &str,
        _hotkey_ss58: &str,
        _from_netuid: NetUid,
        _to_netuid: NetUid,
        _amount: Balance,
    ) -> Result<String> {
        anyhow::bail!("transfer_stake not yet implemented")
    }

    /// Recycle alpha for TAO.
    pub async fn recycle_alpha(
        &self,
        _signer: &sr25519::Pair,
        _hotkey_ss58: &str,
        _netuid: NetUid,
        _amount: u64,
    ) -> Result<String> {
        anyhow::bail!("recycle_alpha not yet implemented")
    }

    /// Claim root dividends.
    pub async fn claim_root(
        &self,
        _signer: &sr25519::Pair,
        _hotkey_ss58: &str,
        _netuid: NetUid,
    ) -> Result<String> {
        anyhow::bail!("claim_root not yet implemented")
    }

    /// Add stake with limit order.
    pub async fn add_stake_limit(
        &self,
        _signer: &sr25519::Pair,
        _hotkey_ss58: &str,
        _netuid: NetUid,
        _amount: Balance,
        _limit_price: u64,
        _allow_partial: bool,
    ) -> Result<String> {
        anyhow::bail!("add_stake_limit not yet implemented")
    }

    /// Remove stake with limit order.
    pub async fn remove_stake_limit(
        &self,
        _signer: &sr25519::Pair,
        _hotkey_ss58: &str,
        _netuid: NetUid,
        _amount: u64,
        _limit_price: u64,
        _allow_partial: bool,
    ) -> Result<String> {
        anyhow::bail!("remove_stake_limit not yet implemented")
    }
}
