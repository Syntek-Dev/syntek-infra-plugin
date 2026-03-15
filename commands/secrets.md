---
description: '[Agent] Manage secrets in Vault and Vaultwarden'
usage: /syntek-infra:secrets [action] [path]
---

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

Spawn the `syntek-infra:vault-manager` agent (model: sonnet) to manage secrets.

## Pre-flight: Run Plugin Tools

Before starting secrets work, gather context using:

- `syntek-infra-tool vault status`
- `syntek-infra-tool vaultwarden status`

## Reference Documents

The agent must read these files from `.claude/` before performing any secrets work:

- `.claude/CODING-PRINCIPLES.md` — coding standards and review checklist
- `.claude/SECURITY.md` — secrets management architecture, Vault policies, agenix bootstrap pattern
- `.claude/ARCHITECTURE-PATTERNS.md` — two-tier secret injection pattern, Vault agent configuration

## The Agent

The agent is a Secrets Management Specialist who:

- Manages secrets in Hashicorp Vault
- Syncs passwords with Vaultwarden
- Generates and rotates Wireguard keys
- Configures agenix/sops-nix for encrypted git secrets

## Actions

- **status** - Check Vault and Vaultwarden connectivity
- **read** - Read a secret from Vault
- **write** - Write a secret to Vault
- **sync** - Sync secrets to Vaultwarden
- **rotate** - Rotate Wireguard keys
- **setup** - Configure agenix or sops-nix

## Required Information

If not provided, the agent will ask:

- **Vault address:** Where is Vault hosted?
- **Authentication:** How to authenticate?
- **Action:** What operation to perform?
- **Path:** Which secret path?

## Example References

The agent uses patterns from:

- `examples/vault/secrets-injection/AGENIX-SECRETS.md` - agenix/sops-nix setup
- `examples/nixos/flakes/BASIC-FLAKE.md` - Flake integration

## Usage Examples

```
/syntek-infra:secrets
/syntek-infra:secrets status
/syntek-infra:secrets read wireguard/devices/laptop
/syntek-infra:secrets rotate wireguard
/syntek-infra:secrets sync vaultwarden
/syntek-infra:secrets setup agenix
```

## User's Request

$ARGUMENTS
