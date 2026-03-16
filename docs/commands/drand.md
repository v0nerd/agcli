# drand — Randomness Beacon Operations

Interact with the Drand randomness beacon pallet. Provides verifiable on-chain randomness from the Drand network.

## Subcommands

### drand write-pulse
Write a verified Drand randomness pulse to the chain.

```bash
agcli drand write-pulse --payload 0x... --signature 0x...
```

- `--payload`: Hex-encoded pulse data from the Drand beacon
- `--signature`: Hex-encoded BLS signature proving the pulse is authentic

Note: This is primarily an infrastructure operation. Most users will consume randomness via runtime queries rather than submitting pulses directly.

## On-chain Pallet
- `Drand::write_pulse(pulses_payload, signature)` — Submit beacon pulse
- `Drand::set_beacon_config(config_payload, signature)` — Root-only config (not wrapped)
