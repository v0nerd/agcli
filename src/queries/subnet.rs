//! Subnet queries.

use crate::chain::Client;
use crate::types::chain_data::{DynamicInfo, SubnetInfo};
use crate::types::NetUid;
use anyhow::Result;

/// List all subnets with basic info.
pub async fn list_subnets(client: &Client) -> Result<Vec<SubnetInfo>> {
    client.get_all_subnets().await
}

/// List all subnets with dynamic (pricing) info.
pub async fn list_dynamic_subnets(client: &Client) -> Result<Vec<DynamicInfo>> {
    client.get_all_dynamic_info().await
}

/// Get details for a specific subnet.
pub async fn subnet_detail(client: &Client, netuid: NetUid) -> Result<Option<SubnetInfo>> {
    client.get_subnet_info(netuid).await
}
