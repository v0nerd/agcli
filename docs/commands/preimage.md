# preimage — Call Preimage Management

Store and manage call preimages on-chain. Preimages are the encoded call data referenced by hash in governance proposals and scheduled calls.

## Subcommands

### preimage note
Store a call preimage on-chain. Returns the preimage hash.

```bash
agcli preimage note --pallet SubtensorModule --call add_stake \
  --args '["5Hotkey...", 1, 1000000000]'
# Output: Preimage hash: 0x...
```

### preimage unnote
Remove a previously stored preimage.

```bash
agcli preimage unnote --hash 0x...
```

## Use Cases
- Store call data for governance proposals
- Prepare calls for scheduler to execute later
- Pre-register complex extrinsic data

## On-chain Pallet
- `Preimage::note_preimage(bytes)` — Store encoded call
- `Preimage::unnote_preimage(hash)` — Remove stored preimage

## Related Commands
- `agcli scheduler schedule` — Schedule a call for future execution
