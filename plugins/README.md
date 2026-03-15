# syntek-infra-tool

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

Rust CLI tool for NixOS and Wireguard environment detection.

## Building

### With Nix (recommended)

```bash
# Enter development shell
nix develop

# Build
cargo build --release

# Or build with Nix
nix build
```

### With Cargo

```bash
cargo build --release
```

The binary will be at `target/release/syntek-infra-tool`.

## Usage

### Nix Commands

```bash
# Detect Nix installation
syntek-infra-tool nix detect

# List channels
syntek-infra-tool nix channels

# Validate a flake
syntek-infra-tool nix flake-check --path /path/to/flake
```

### NixOS Commands

```bash
# Get system status
syntek-infra-tool nixos status

# List generations
syntek-infra-tool nixos generations

# Validate configuration
syntek-infra-tool nixos validate --path /path/to/config
```

### Vault Commands

```bash
# Check Vault status
export VAULT_ADDR=https://vault.example.com:8200
syntek-infra-tool vault status

# Read a secret
syntek-infra-tool vault read --path secret/wireguard/laptop

# Write a secret
syntek-infra-tool vault write --path secret/test --data '{"key": "value"}'
```

### Vaultwarden Commands

```bash
# Check Vaultwarden status
export VAULTWARDEN_URL=https://vw.example.com
syntek-infra-tool vaultwarden status

# Sync (placeholder)
syntek-infra-tool vaultwarden sync
```

### Wireguard Commands

```bash
# Generate key pair
syntek-infra-tool wireguard keygen

# Generate QR code
syntek-infra-tool wireguard qr --config /path/to/wg0.conf
```

## Output Format

All commands output JSON for easy parsing by agents:

```json
{
  "installed": true,
  "version": "nix (Nix) 2.18.1",
  "flakes_enabled": true,
  "nix_command_enabled": true,
  "multi_user": true,
  "channels": ["nixos"]
}
```

## Dependencies

- `clap` - CLI argument parsing
- `serde` / `serde_json` - JSON serialization
- `anyhow` - Error handling
- `tokio` - Async runtime (for future HTTP calls)
- `reqwest` - HTTP client (for Vault/Vaultwarden API)

## Development

```bash
# Enter Nix development shell
nix develop

# Run tests
cargo test

# Run with arguments
cargo run -- nix detect
```
