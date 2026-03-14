# Staking Guide

## Understanding Staking in Bittensor

Staking in Bittensor means locking TAO tokens on a subnet to support validators/miners.
With Dynamic TAO, each subnet has its own alpha token — when you stake, you buy alpha.

## Staking Wizard (Interactive)

The quickest way to start staking:

```bash
agcli stake wizard
```

The wizard shows your balance, lists top subnets by pool size, lets you pick a subnet and amount, and confirms before submitting.

## Basic Staking

```bash
# Stake 100 TAO on subnet 1 using your default hotkey
agcli stake add 100.0 --netuid 1

# Stake on a specific hotkey
agcli stake add 50.0 --netuid 1 --hotkey 5HotkeyAddress...

# View all your stakes
agcli stake list
```

## Unstaking

```bash
# Remove some stake
agcli stake remove 25.0 --netuid 1

# Unstake everything from a hotkey
agcli stake unstake-all --hotkey 5HotkeyAddress...
```

## Moving Stake Between Subnets

```bash
# Move 10 alpha from subnet 1 to subnet 3
agcli stake move 10.0 --from 1 --to 3
```

## Swapping Stake Between Hotkeys

```bash
# Move 5 alpha from hotkey A to hotkey B on subnet 1
agcli stake swap 5.0 --netuid 1 --from-hotkey 5A... --to-hotkey 5B...
```

## Limit Orders

```bash
# Add stake only if price <= 0.5 TAO per alpha
agcli stake add-limit 100.0 --netuid 1 --price 0.5

# Allow partial fills
agcli stake add-limit 100.0 --netuid 1 --price 0.5 --partial
```

## Claiming Root Dividends

```bash
agcli stake claim-root --netuid 1
```

## Delegate Take Management

```bash
# View current take
agcli delegate show

# Decrease take (takes effect immediately)
agcli delegate decrease-take 10.0

# Increase take (rate-limited)
agcli delegate increase-take 12.0
```

## Childkey Delegation

```bash
# Set childkey take to 5%
agcli stake childkey-take 5.0 --netuid 1

# Delegate to children
agcli stake set-children --netuid 1 --children "50:5Child1...,50:5Child2..."
```
