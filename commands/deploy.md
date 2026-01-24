---
description: '[Agent] Deploy NixOS configuration to target device'
usage: /syntek-infra:deploy [device] [target]
---

Spawn the `syntek-infra:infra-architect` agent (model: sonnet) to deploy NixOS
configurations.

## Pre-flight: Run Plugin Tools

Before deploying, gather context using:

- `syntek-infra-tool nix detect`
- `syntek-infra-tool nixos status`
- `syntek-infra-tool vault status`

## The Agent

The agent is an Infrastructure Architect who:

- Validates configurations before deployment
- Coordinates secret injection from Vault
- Executes deployment (with user approval)
- Monitors deployment success
- Handles rollback on failure

## Deployment Modes

- **local** - Deploy to the current machine using `nixos-rebuild switch`
- **remote** - Deploy to a remote machine via SSH
- **dry-run** - Validate without applying changes

## Required Information

If not provided, the agent will ask:

- **Device:** Which device configuration to deploy?
- **Target:** Local or remote (IP/hostname)?
- **Mode:** switch, boot, test, or dry-run?

## IMPORTANT

The agent will **always wait for user approval** before executing any
deployment. It will:

1. Show the configuration to be deployed
2. Explain what changes will be made
3. Ask for explicit confirmation
4. Only proceed after approval

## Example References

The agent uses patterns from:

- `examples/nixos/flakes/BASIC-FLAKE.md` - Flake structure
- `examples/vault/secrets-injection/AGENIX-SECRETS.md` - Secrets handling

## Usage Examples

```
/syntek-infra:deploy
/syntek-infra:deploy laptop local
/syntek-infra:deploy laptop dry-run
/syntek-infra:deploy server 192.168.1.10
/syntek-infra:deploy router remote
```

## User's Request

$ARGUMENTS
