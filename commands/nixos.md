---
description: '[Agent] Generate NixOS configuration'
usage: /syntek-infra:nixos [device] [profile]
---

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

Spawn the `syntek-infra:nixos-builder` agent (model: sonnet) to generate NixOS
configurations.

## Pre-flight: Run Plugin Tools

Before starting NixOS work, gather context using:

- `syntek-infra-tool nix detect`
- `syntek-infra-tool nixos status`

## Reference Documents

The agent must read these files from `.claude/` before producing any configuration:

- `.claude/CODING-PRINCIPLES.md` — coding standards and review checklist
- `.claude/SECURITY.md` — secrets management, systemd hardening, security checklist
- `.claude/ARCHITECTURE-PATTERNS.md` — module system, flake structure, device profiles, systemd patterns
- `.claude/PERFORMANCE.md` — build performance, binary caches, service tuning
- `.claude/DATA-STRUCTURES.md` — Nix option types, configuration modelling
- `.claude/TESTING.md` — NixOS VM tests, `nix flake check` requirements

## The Agent

The agent is a NixOS Configuration Specialist who:

- Generates NixOS configurations using Flakes
- Validates configurations before deployment
- Debugs build errors
- Optimises configurations for performance

## Required Information

If not provided, the agent will ask:

- **Device type:** laptop, server, router, mobile, cloud
- **Profile:** minimal or full (batteries-included)
- **Hardware details:** CPU, GPU, storage
- **Services:** Docker, Wireguard, development tools

## Example References

The agent uses patterns from:

- `examples/nixos/flakes/BASIC-FLAKE.md` - Flake structure
- `examples/nixos/hardware/INTEL-LAPTOP.md` - Hardware configuration
- `examples/vault/secrets-injection/AGENIX-SECRETS.md` - Secrets handling

## Usage Examples

```
/syntek-infra:nixos
/syntek-infra:nixos laptop
/syntek-infra:nixos laptop full
/syntek-infra:nixos server minimal
```

## User's Request

$ARGUMENTS
