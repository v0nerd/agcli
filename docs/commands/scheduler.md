# scheduler — Schedule Future Calls

Schedule extrinsics for execution at a future block. Supports one-time and periodic scheduling.

## Subcommands

### scheduler schedule
Schedule a call for a specific block.

```bash
agcli scheduler schedule --when 100000 \
  --pallet SubtensorModule --call add_stake \
  --args '["5Hotkey...", 1, 1000000000]' \
  --priority 128
```

- `--when`: Target block number
- `--priority`: 0=highest, 255=lowest (default 128)
- `--repeat-every N --repeat-count M`: Execute every N blocks, M times

### scheduler schedule-named
Schedule a named call (can be cancelled by name).

```bash
agcli scheduler schedule-named --id "my-stake-task" --when 100000 \
  --pallet SubtensorModule --call add_stake \
  --args '["5Hotkey...", 1, 1000000000]'
```

### scheduler cancel
Cancel a scheduled task by block and index.

```bash
agcli scheduler cancel --when 100000 --index 0
```

### scheduler cancel-named
Cancel a named scheduled task.

```bash
agcli scheduler cancel-named --id "my-stake-task"
```

## Use Cases
- Schedule stake operations for a specific block
- Time-delayed extrinsics ("add stake in 100 blocks")
- Periodic operations (e.g., auto-restake every 1000 blocks)

## On-chain Pallet
- `Scheduler::schedule` / `Scheduler::schedule_named`
- `Scheduler::cancel` / `Scheduler::cancel_named`

## Related Commands
- `agcli preimage note` — Store call data for governance/scheduler
- `agcli batch` — Execute multiple calls now (vs scheduling for later)
