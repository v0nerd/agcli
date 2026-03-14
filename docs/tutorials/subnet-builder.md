# Subnet Builder Guide

## Creating a Subnet

Registering a new subnet costs TAO (lock cost that decreases over time).

```bash
# Register a new subnet
agcli subnet register

# Register with identity
# (Sets name, GitHub, description on-chain)
agcli identity set-subnet 42 --name "MySubnet" --github "user/repo" --url "https://example.com"
```

## Registering Neurons

### Burn Registration
```bash
# Register by burning TAO
agcli subnet register-neuron 42
```

### POW Registration
```bash
# Register via proof-of-work (uses multiple CPU threads)
agcli subnet pow 42 --threads 8
```

## Serving

Once registered, miners need to set their axon endpoint:

```rust
use agcli::{Client, types::chain_data::AxonInfo};

let axon = AxonInfo {
    block: 0,
    version: 1,
    ip: "1.2.3.4".to_string(),
    port: 8091,
    ip_type: 4,
    protocol: 0,
};
client.serve_axon(&signer, 42.into(), &axon).await?;
```

## Setting Weights (Validators)

Validators rank miners by setting weights:

```bash
# Set weights: miner UID 0 gets weight 100, UID 1 gets 200
agcli weights set --netuid 42 "0:100,1:200"
```

### Commit-Reveal Weights
If the subnet uses commit-reveal:

```bash
# Step 1: Commit
agcli weights commit --netuid 42 "0:100,1:200"

# Step 2: Reveal (after required interval)
agcli weights reveal --netuid 42 "0:100,1:200" SALT_FROM_COMMIT
```

## Subnet Hyperparameters

```bash
# View hyperparameters
agcli subnet hyperparams 42
```

Key parameters:
- `tempo` — blocks between epochs
- `min_allowed_weights` — minimum number of weight entries
- `max_weights_limit` — max weight per entry
- `immunity_period` — blocks before a new neuron can be pruned
- `min_burn` / `max_burn` — registration cost range
- `commit_reveal_weights_enabled` — whether weights must use commit-reveal
