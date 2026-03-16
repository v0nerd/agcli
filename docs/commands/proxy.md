# proxy — Proxy Account Management

Delegate signing authority to another account. Proxy accounts can sign transactions on behalf of the delegator, filtered by operation type.

## Subcommands

### proxy add
Add a proxy delegate.

```bash
agcli proxy add --delegate SS58 [--proxy-type staking] [--delay 0]
```

### proxy remove
Remove a proxy delegate.

```bash
agcli proxy remove --delegate SS58 [--proxy-type staking] [--delay 0]
```

### proxy list
List all proxy delegates for an account.

```bash
agcli proxy list [--address SS58]
# JSON: [{"delegate", "proxy_type", "delay"}]
```

### proxy create-pure
Create a pure (anonymous) proxy account.

```bash
agcli proxy create-pure [--proxy-type any] [--delay 0] [--index 0]
```

### proxy kill-pure
Destroy a pure proxy account. **WARNING: funds become permanently inaccessible!**

```bash
agcli proxy kill-pure --spawner SS58 --height BLOCK --ext-index IDX
```

### proxy announce
Announce a proxy call for time-delayed execution. Used before `proxy-announced`.

```bash
agcli proxy announce --real 5RealAccount... --call-hash 0x...
```

### proxy proxy-announced
Execute a previously announced proxy call after the delay period.

```bash
agcli proxy proxy-announced --delegate 5Delegate... --real 5Real... \
  --pallet SubtensorModule --call add_stake --args '[...]' \
  [--proxy-type staking]
```

### proxy reject-announcement
Reject an announced proxy call (called by the real account).

```bash
agcli proxy reject-announcement --delegate 5Delegate... --call-hash 0x...
```

### proxy list-announcements
List pending proxy announcements for an account.

```bash
agcli proxy list-announcements [--address SS58]
```

## Proxy Types
| Type | Allowed Operations |
|------|-------------------|
| `any` | All operations |
| `owner` | Subnet owner operations |
| `staking` | Stake add/remove/move only |
| `non_transfer` | Everything except transfers |
| `non_critical` | Non-critical operations |
| `governance` | Governance voting |
| `senate` | Senate operations |
| `transfer` | Transfer operations only |
| `registration` | Registration operations |
| `root_weights` | Root weight setting |
| `child_keys` | Child key operations |

## Time-Delayed Proxy Workflow
1. `proxy add --delegate D --delay 100` — Add proxy with 100 block delay
2. `proxy announce --real R --call-hash 0x...` — Delegate announces intent
3. Wait 100 blocks
4. `proxy proxy-announced --delegate D --real R --pallet P --call C` — Execute

## On-chain Pallet
- `Proxy::add_proxy` / `Proxy::remove_proxy`
- `Proxy::create_pure` / `Proxy::kill_pure`
- `Proxy::announce` / `Proxy::proxy_announced` / `Proxy::reject_announcement`

## Related Commands
- `agcli multisig` — Multi-party approval (vs single-signer delegation)
