# Agents

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

This directory contains agent definitions for the syntek-infra plugin.

## Available Agents

| Agent                   | Description                                                     | Model  |
| ----------------------- | --------------------------------------------------------------- | ------ |
| `infra-architect`       | High-level infrastructure planning and deployment orchestration | sonnet |
| `nixos-builder`         | NixOS configuration generation and validation specialist        | sonnet |
| `network-engineer`      | Network and Wireguard VPN configuration specialist              | sonnet |
| `vault-manager`         | Secrets management with Hashicorp Vault and Vaultwarden         | sonnet |
| `hyprland-configurator` | Hyprland wayland compositor configuration specialist            | sonnet |

## Agent Responsibilities

### infra-architect

The architect agent is the entry point for complex infrastructure tasks. It:

- Gathers requirements from the user
- Designs network topology
- Coordinates other agents
- Orchestrates multi-device deployments

### nixos-builder

The builder agent handles NixOS configuration:

- Generates Flake-based configurations
- Validates configurations before deployment
- Debugs build errors
- Optimises configurations

### network-engineer

The network agent handles Wireguard and networking:

- Designs VPN topologies
- Configures all four Wireguard use cases
- Sets up firewall rules
- Generates QR codes for mobile

### vault-manager

The vault agent handles secrets:

- Manages Hashicorp Vault secrets
- Syncs with Vaultwarden
- Generates and rotates Wireguard keys
- Configures agenix/sops-nix

### hyprland-configurator

The Hyprland agent handles Wayland compositor configuration:

- Generates custom hyprland.conf configurations
- Configures keybindings, animations, and window rules
- Sets up multi-monitor layouts
- Integrates Hyprland ecosystem (waybar, wofi, dunst)
- Debugs Hyprland issues and optimises performance

## Usage

Agents are invoked via commands:

```
/syntek-infra:nixos      → nixos-builder agent
/syntek-infra:wireguard  → network-engineer agent
/syntek-infra:secrets    → vault-manager agent
/syntek-infra:deploy     → infra-architect agent
/syntek-infra:init       → infra-architect agent
/syntek-infra:hyprland   → hyprland-configurator agent
```
