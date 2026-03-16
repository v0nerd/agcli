# batch — Batch Extrinsic Submission

Submit multiple extrinsics from a JSON file. Supports atomic, non-atomic, and force modes.

## Usage

```bash
agcli batch --file calls.json [--no-atomic] [--force] [--yes]
```

## JSON Format
```json
[
  {"pallet": "SubtensorModule", "call": "add_stake", "args": ["5Hotkey...", 1, 1000000000]},
  {"pallet": "Balances", "call": "transfer_allow_death", "args": ["5Dest...", 5000000000]},
  {"pallet": "SubtensorModule", "call": "set_weights", "args": [1, [0,1], [100,200], 0]}
]
```

- Hex strings in args (`"0xdead..."`) are auto-decoded as bytes
- Uses `submit_raw_call` for each call — any pallet/call combo works

## Batch Modes

| Flag | Utility Call | Behavior |
|------|-------------|----------|
| (default) | `batch_all` | **Atomic**: All calls succeed or all revert |
| `--no-atomic` | `batch` | **Non-atomic**: Failed calls don't revert others, but may stop on first failure |
| `--force` | `force_batch` | **Force**: Continues execution even if individual calls fail, never reverts |

### When to use each:
- **batch_all** (default): Related operations that must all succeed together
- **batch** (`--no-atomic`): Independent operations where you want to know which failed
- **force_batch** (`--force`): Best-effort execution, never reverts successful calls

## On-chain Pallet
- `Utility::batch_all` — Atomic batch
- `Utility::batch` — Non-atomic batch
- `Utility::force_batch` — Force batch (continues on failure)

## Related Commands
- `agcli scheduler schedule` — Schedule calls for future blocks
