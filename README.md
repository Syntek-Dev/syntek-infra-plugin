# syntek-infra-plugin

**Last Updated:** 05/04/2026
**Version:** 2.1.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

A Claude Code plugin for NixOS configuration and Wireguard VPN setup assistance.

## Overview

This plugin provides agents, commands, skills, and tools to help with:

- **NixOS Configuration** - Generate Flake-based configurations for various
  devices
- **Wireguard VPN** - Set up VPNs for privacy, remote access, and network
  routing
- **Secrets Management** - Integrate with Hashicorp Vault and Vaultwarden

## Installation

Add this plugin to your Claude Code plugins directory or install via the
marketplace.

## Agents and Commands

### Infrastructure Architect (`/syntek-infra:init`, `/syntek-infra:deploy`)

The Infrastructure Architect is your high-level planning and orchestration
specialist. Use this agent when starting a new NixOS project or planning
multi-device deployments. It helps you design network topologies, select
appropriate device profiles, plan Wireguard VPN configurations, and coordinate
deployments across multiple machines. The agent gathers your requirements,
creates implementation plans, and hands off to specialised agents for the actual
configuration work. It always waits for your approval before executing any
deployment.

### NixOS Builder (`/syntek-infra:nixos`)

The NixOS Builder is your configuration generation specialist. Use this agent
when you need to create or modify NixOS configurations using Flakes. It
generates complete, validated configurations for laptops, servers, routers, and
other devices. The agent detects your current environment, helps you choose
between minimal and full profiles, composes modules from the examples library,
and validates configurations with `nix flake check` before presenting them. It
handles hardware detection, service configuration, and user setup.

### Network Engineer (`/syntek-infra:wireguard`)

The Network Engineer handles all Wireguard VPN configurations and network
security. Use this agent when setting up any of the four VPN use cases: outbound
privacy VPN via Mullvad, site-to-site client access via Raspberry Pi, road
warrior remote access to home devices, or full tunnel routing through your
server. It generates key pairs, creates server and client configurations, sets
up firewall rules, and can generate QR codes for mobile device setup. The agent
integrates with Vault for secure key storage.

### Vault Manager (`/syntek-infra:secrets`)

The Vault Manager is your secrets management specialist. Use this agent when
working with Hashicorp Vault, Vaultwarden, or encrypted secrets in git. It helps
you store and retrieve Wireguard keys, API tokens, and service credentials. The
agent can set up agenix or sops-nix for encrypted secrets in your repository,
sync passwords to Vaultwarden for Bitwarden client access, and plan secret
rotation strategies. It ensures secrets are never hardcoded in your NixOS
configurations.

## Supported Devices

- **Laptops** - Developer workstations with Zed, Docker, VPN
- **Servers** - Home/business servers with Docker, Vault, monitoring
- **Routers** - DIY routers with firewall, NAT, DHCP, DNS
- **Cloud** - Cloud servers on Hetzner, AWS, DigitalOcean
- **Mobile** - Pinephone and Pinetab devices

## Wireguard Use Cases

1. **Outbound VPN** - Privacy via Mullvad with SSH proxy rotation
2. **Client Access** - VM into client environments via Raspberry Pi
3. **Home Access** - Remote access to home devices (road warrior)
4. **Full Tunnel** - Route all device traffic through home server

## Technical Decisions

- **NixOS stable** channel (not unstable)
- **Nix Flakes** for configuration management
- **Pure Nix** language only
- **Hashicorp Vault** for configuration secrets
- **Vaultwarden** for password management
- **agenix/sops-nix** for encrypted secrets in git

## Examples Coverage

The plugin includes comprehensive examples for:

| Topic                   | Description                                |
| ----------------------- | ------------------------------------------ |
| NixOS modules & flakes  | Flake structure, custom modules            |
| Hardware configurations | Intel, AMD (CPU and GPU), headless servers |
| Firewall rules          | nftables with zones, rate limiting, NAT    |
| systemd services        | Service deployment, timers, hardening      |
| ZFS filesystems         | Pools, datasets, snapshots, encryption     |
| Network segmentation    | VLANs, bridges, wireless isolation         |
| Wireguard VPN           | Mullvad client, full tunnel server         |
| Secrets management      | agenix and sops-nix integration            |

## Plugin Structure

```
syntek-infra-plugin/
├── .claude-plugin/      # Plugin metadata
├── agents/              # Agent definitions
│   ├── infra-architect.md
│   ├── nixos-builder.md
│   ├── network-engineer.md
│   └── vault-manager.md
├── commands/            # Slash commands
│   ├── init.md
│   ├── nixos.md
│   ├── wireguard.md
│   ├── secrets.md
│   └── deploy.md
├── skills/              # Device-specific guidance
│   ├── global-workflow/
│   ├── device-laptop/
│   ├── device-server/
│   ├── device-router/
│   └── device-mobile/
├── plugins/             # Rust CLI tool
│   ├── src/
│   ├── Cargo.toml
│   └── flake.nix
├── templates/           # Project templates
├── examples/            # Code examples
└── docs/                # Documentation
```

## Building the Rust Tool

```bash
cd plugins

# With Nix
nix build

# With Cargo
cargo build --release
```

## Related Plugins

- [syntek-dev-suite](https://github.com/Syntek-Studio/syntek-dev-suite) -
  Development workflow tooling
- [syntek-rust-security](https://github.com/Syntek-Studio/syntek-rust-security) -
  Rust security tooling

## Licence

MIT
