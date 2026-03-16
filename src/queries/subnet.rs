//! Subnet queries.

use crate::chain::Client;
use crate::types::chain_data::SubnetInfo;
use anyhow::Result;

/// List all subnets with basic info.
/// Fetches subnets and dynamic info concurrently, then enriches subnet names.
pub async fn list_subnets(client: &Client) -> Result<Vec<SubnetInfo>> {
    // Fetch subnets and dynamic info concurrently (both are cached independently)
    let (subnets_arc, dynamic_result) =
        tokio::join!(client.get_all_subnets(), client.get_all_dynamic_info(),);
    let mut subnets = (*subnets_arc?).clone();
    // Enrich subnet list with real names from DynamicInfo (one call vs N identity queries)
    if let Ok(dynamic) = dynamic_result {
        let name_map: std::collections::HashMap<u16, (String, u64)> = dynamic
            .iter()
            .filter(|d| !d.name.is_empty())
            .map(|d| (d.netuid.0, (d.name.clone(), d.total_emission())))
            .collect();
        for s in &mut subnets {
            if let Some((name, emission)) = name_map.get(&s.netuid.0) {
                s.name = name.clone();
                if s.emission_value == 0 {
                    s.emission_value = *emission;
                }
            }
        }
    }
    Ok(subnets)
}
