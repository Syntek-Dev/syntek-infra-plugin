---
description: '[Agent] Configure Wireguard VPN'
usage: /syntek-infra:wireguard [use-case] [device]
---

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

Spawn the `syntek-infra:network-engineer` agent (model: sonnet) to configure
Wireguard VPN.

## Pre-flight: Run Plugin Tools

Before starting Wireguard work, gather context using:

- `syntek-infra-tool vault status`
- `syntek-infra-tool wireguard keygen` (when generating new keys)

## Reference Documents

The agent must read these files from `.claude/` before producing any configuration:

- `.claude/CODING-PRINCIPLES.md` — coding standards and review checklist
- `.claude/SECURITY.md` — Wireguard key handling, secrets management, firewall rules
- `.claude/ARCHITECTURE-PATTERNS.md` — Wireguard topologies, secret injection patterns
- `.claude/DATA-STRUCTURES.md` — Nix option types for network configuration

## The Agent

The agent is a Network Engineer who:

- Designs Wireguard VPN topologies
- Configures all four Wireguard use cases
- Sets up firewall and routing rules
- Generates QR codes for mobile devices

## Wireguard Use Cases

1. **outbound-vpn** - Privacy via Mullvad with SSH proxy rotation
2. **client-access** - VM into client environments via Raspberry Pi
3. **home-access** - Remote access to home devices (road warrior)
4. **full-tunnel** - Route all traffic through home/business server

## Required Information

If not provided, the agent will ask:

- **Use case:** Which VPN scenario?
- **Device:** Which device is being configured?
- **Server details:** Endpoint, public key (for client configs)
- **Network topology:** What needs to be accessible?

## Example References

The agent uses patterns from:

- `examples/wireguard/outbound-vpn/MULLVAD-CLIENT.md` - Mullvad setup
- `examples/wireguard/full-tunnel/SERVER-CONFIG.md` - Full tunnel server
- `examples/vault/secrets-injection/AGENIX-SECRETS.md` - Key storage

## Usage Examples

```
/syntek-infra:wireguard
/syntek-infra:wireguard outbound-vpn
/syntek-infra:wireguard home-access phone
/syntek-infra:wireguard client-access laptop
/syntek-infra:wireguard full-tunnel
```

## User's Request

$ARGUMENTS
