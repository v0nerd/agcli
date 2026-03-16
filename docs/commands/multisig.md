# multisig — Multisig Operations

Create and manage multi-signature transactions. Requires M-of-N signatories to approve before execution.

## Subcommands

### multisig address
Derive a deterministic multisig address from signatories and threshold.

```bash
agcli multisig address --signatories "SS58_1,SS58_2,SS58_3" --threshold 2
# Output: multisig SS58 address
```

### multisig submit
Submit a new multisig call (first approval via approve_as_multi).

```bash
agcli multisig submit --others "SS58_2,SS58_3" --threshold 2 \
  --pallet SubtensorModule --call add_stake --args '[...]'
```

### multisig approve
Approve a pending multisig call by its hash.

```bash
agcli multisig approve --others "SS58_2,SS58_3" --threshold 2 --call-hash 0x...
```

### multisig execute
Execute a multisig call (as_multi). The final signatory uses this to actually execute the underlying call once threshold is met.

```bash
agcli multisig execute --others "SS58_2,SS58_3" --threshold 2 \
  --pallet SubtensorModule --call add_stake --args '[...]' \
  --timepoint-height 12345 --timepoint-index 0
```

- `--timepoint-height` / `--timepoint-index`: From `multisig list` output. Optional for first call.

### multisig cancel
Cancel a pending multisig operation. Only the original submitter can cancel.

```bash
agcli multisig cancel --others "SS58_2,SS58_3" --threshold 2 \
  --call-hash 0x... --timepoint-height 12345 --timepoint-index 0
```

### multisig list
List pending multisig operations for a multisig account.

```bash
agcli multisig list --address 5MultisigSS58...
```

Output: call hash, timepoint (height/index), approval count, deposit.

## Full Workflow

1. **Derive address**: `multisig address` to get the multisig account SS58
2. **Fund**: Transfer TAO to the multisig address
3. **Submit**: First signer calls `multisig submit` (proposes + first approval)
4. **Approve**: Other signers call `multisig approve` with the call hash
5. **Execute**: Final signer calls `multisig execute` with the full call data and timepoint
6. **Monitor**: Use `multisig list` to check pending operations

## On-chain Pallets
- `Multisig::approve_as_multi` — submit/approve
- `Multisig::as_multi` — execute (final approval with call data)
- `Multisig::cancel_as_multi` — cancel pending

## Related Commands
- `agcli proxy add` — Simpler delegation (single signer)
