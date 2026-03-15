# CLAUDE.md Template

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

## Overview

Template for the `.claude/CLAUDE.md` file in NixOS configuration projects initialised by the syntek-infra plugin. This file provides project context for Claude Code and all agents.

## Metadata

| Property            | Value                                 |
| ------------------- | ------------------------------------- |
| **Template Version** | 2.0.0                                |
| **Last Updated**    | 15/03/2026                            |
| **Plugin**          | syntek-infra                          |
| **Stack**           | NixOS, Hyprland, Wireguard, Vault     |

---

## Table of Contents

- [Overview](#overview)
- [Metadata](#metadata)
- [Template](#template)
- [Settings File Template](#settings-file-template)

---

## Template

```markdown
# [Hostname] — NixOS Configuration

## Stack
- **OS:** NixOS [version] (stable)
- **Configuration:** Nix Flakes
- **Device Type:** [laptop | server | router | cloud | mobile]
- **Architecture:** [x86_64-linux | aarch64-linux]
- **Locale:** en_GB
- **Timezone:** Europe/London
- **Secrets:** Hashicorp Vault + agenix (bootstrap only)

## Coding Principles

All code in this project follows the principles in `.claude/CODING-PRINCIPLES.md`, organised in layers:

- **Low-level coding instincts:** Rob Pike's 5 Rules and Linus Torvalds' Coding Rules — measure before optimising, simple data structures over fancy algorithms, data models drive logic, short focused functions, favour stability over cleverness
- **Code quality check:** CUPID Properties — composable, Unix philosophy, predictable, idiomatic, domain-based
- **Everyday decisions:** DRY (Rule of Three), KISS, YAGNI — do not abstract until three occurrences, keep it simple, do not build for hypothetical requirements

Additional sections cover the 750-line file limit, error handling (Rust `Result` types, Nix `lib.mkAssert`), naming conventions (Nix `camelCase` options, Rust `snake_case`), testing requirements, security (Vault-first secrets), dependencies, git workflow, and the code review checklist.

See `.claude/CODING-PRINCIPLES.md` for the full rules.

## Required Reference Documents

All agents MUST read these documents before writing Nix modules, Rust code, or performing reviews:

| Document | Purpose |
|----------|---------|
| `.claude/CODING-PRINCIPLES.md` | Software design principles (CUPID, Pike, Torvalds), error handling, naming conventions (Nix and Rust), testing requirements, security, dependencies, git workflow, and code review checklist |
| `.claude/TESTING.md` | Testing guide: Rust unit tests (`cargo test`, `mockall`), integration tests (`assert_cmd`, `wiremock`), NixOS VM tests (`nixosTest`), property-based tests (`proptest`), smoke tests, TDD cycle, mocking patterns |
| `.claude/SECURITY.md` | Secrets management (agenix bootstrap + Vault for all other secrets), Wireguard key handling (never in config, always in Vault), NixOS systemd service hardening, SSH access control, Vault policy least-privilege, and security checklist |
| `.claude/DEVELOPMENT.md` | Development workflow: prerequisites, getting started, local dev loop, Rust tool development, Hyprland config testing, Wireguard key operations, deployment (nixos-rebuild, nixos-anywhere), rollback, common tasks, and troubleshooting |
| `.claude/ARCHITECTURE-PATTERNS.md` | NixOS module system design, flake structure patterns, device profile pattern, systemd service patterns (hardening, secret injection, ordering), secret injection architecture (two-tier agenix+Vault), Wireguard topologies, overlay patterns, Hyprland configuration patterns |
| `.claude/PERFORMANCE.md` | Nix build performance (binary caches, distributed builds), evaluation performance (avoiding IFD), Nix store management (GC, optimisation), systemd service tuning (startup time, resource limits), Wireguard performance, Vault performance, monitoring |
| `.claude/DATA-STRUCTURES.md` | Nix type system (primitives, attrsets, lists, derivations), NixOS option types (scalar, collection, submodule, custom), module option design, Rust newtypes, enums and error types, configuration data modelling, anti-patterns |

## Project Structure
```
[hostname]/
├── .claude/
│   ├── CLAUDE.md                    # This file
│   ├── CODING-PRINCIPLES.md
│   ├── TESTING.md
│   ├── SECURITY.md
│   ├── DEVELOPMENT.md
│   ├── ARCHITECTURE-PATTERNS.md
│   ├── PERFORMANCE.md
│   └── DATA-STRUCTURES.md
├── flake.nix                        # Flake inputs and outputs
├── flake.lock                       # Locked dependencies (commit this)
├── configuration.nix                # Main system configuration
├── hardware-configuration.nix       # Hardware-specific (do not edit manually)
├── hosts/                           # Per-host configurations
│   └── [hostname]/
│       └── configuration.nix
├── profiles/                        # Reusable device profiles
│   ├── base.nix
│   ├── workstation.nix              # Laptops: Hyprland, audio, Bluetooth
│   └── server.nix                   # Servers: headless, sshd, firewall
├── modules/                         # Custom NixOS modules
│   ├── wireguard/
│   │   ├── client.nix
│   │   └── server.nix
│   ├── vault/
│   │   └── agent.nix
│   └── hyprland/
│       └── default.nix
├── secrets/                         # agenix-encrypted secrets
│   ├── secrets.nix                  # Public key declarations
│   └── vault-token.age              # Bootstrap token (the only agenix secret)
├── tests/
│   └── nixos/                       # NixOS VM tests
│       └── wireguard.nix
└── plugins/                         # Rust CLI tools (syntek-infra-tool)
    └── syntek-infra-tool/
```

## Available Commands

| Command | Purpose |
|---------|---------|
| `/syntek-infra:nixos` | Generate or update NixOS configuration |
| `/syntek-infra:hyprland` | Configure Hyprland compositor |
| `/syntek-infra:wireguard` | Add or update Wireguard VPN configuration |
| `/syntek-infra:secrets` | Manage secrets in Vault/Vaultwarden |
| `/syntek-infra:deploy` | Deploy configuration to target device |

## Development Commands

```bash
# Enter development shell (provides all required tools)
nix develop

# Validate flake (fast, no build)
nix flake check --no-build

# Full flake check (with VM tests)
nix flake check

# Build system closure (catches build errors before deploying)
nix build .#nixosConfigurations.[hostname].config.system.build.toplevel

# Test in VM
nixos-rebuild build-vm --flake .#[hostname]
./result/bin/run-[hostname]-vm

# Deploy to local machine
sudo nixos-rebuild switch --flake .#[hostname]

# Deploy to remote machine (via VPN)
nixos-rebuild switch --flake .#[hostname] --target-host admin@[hostname].vpn --use-remote-sudo

# Rust tool development
cd plugins/syntek-infra-tool
cargo test --workspace
cargo clippy -- -D warnings
cargo fmt --check
```

## Key Decisions

- **NixOS stable** — not unstable. Stability over bleeding edge.
- **Nix Flakes** — modern configuration management with pinned inputs.
- **Pure Nix** — configuration in Nix language only. No YAML or JSON config files.
- **Vault-first secrets** — all secrets in Hashicorp Vault. The only exception is the Vault bootstrap token, encrypted with agenix.
- **Wireguard VPN** — all service-to-service and admin access over VPN. No services exposed to the public internet except the Wireguard UDP port.
- **Declarative rollback** — NixOS generations allow rolling back to any previous configuration with `nixos-rebuild --rollback`.

## Environment

- **Vault address:** `[vault-address]`
- **VPN subnet:** `[wireguard-subnet]`
- **Build host:** `[build-host or "local"]`
```

---

## Settings File Template

### .claude/settings.local.json

```json
{
  "language": "en_GB",
  "locale": "en_GB",
  "timezone": "Europe/London",
  "permissions": {
    "allow": [
      "Read(**)",
      "Edit(**)",
      "Write(**)",
      "Bash(nix:*)",
      "Bash(nixos-rebuild:*)",
      "Bash(cargo:*)",
      "Bash(wg:*)",
      "Bash(vault:*)",
      "Bash(agenix:*)",
      "Bash(git:*)",
      "Bash(systemctl:*)",
      "Bash(journalctl:*)"
    ],
    "deny": [
      "Bash(rm -rf:*)",
      "Bash(nixos-rebuild switch:*)"
    ]
  }
}
```

Note: `nixos-rebuild switch` is in the deny list because deployments must be explicitly approved by the user. The agent uses `build` and `build-vm` freely, but `switch` (which actually modifies the running system) requires user confirmation.
