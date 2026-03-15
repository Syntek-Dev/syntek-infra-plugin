---
description: '[Agent] Initialise a new NixOS configuration project'
usage: /syntek-infra:init [template]
---

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

Spawn the `syntek-infra:infra-architect` agent (model: sonnet) to initialise a
new NixOS configuration project.

## Pre-flight: Run Plugin Tools

Before initialising, gather context using:

- `syntek-infra-tool nix detect`

## Reference Documents

After copying the seven documentation files into `.claude/`, the agent must read all of them before generating any configuration:

- `.claude/CODING-PRINCIPLES.md`
- `.claude/SECURITY.md`
- `.claude/TESTING.md`
- `.claude/DEVELOPMENT.md`
- `.claude/ARCHITECTURE-PATTERNS.md`
- `.claude/PERFORMANCE.md`
- `.claude/DATA-STRUCTURES.md`

## The Agent

The agent is an Infrastructure Architect who:

- Helps choose the right template for your device
- Creates the initial project structure
- Configures CLAUDE.md with project context
- Sets up the Flake structure

## Available Templates

| Template | Description                                  |
| -------- | -------------------------------------------- |
| `laptop` | Developer laptop with Zed, Docker, Wireguard |
| `server` | Home/business server with services           |
| `router` | DIY router with firewall, NAT, DNS           |
| `cloud`  | Cloud server (Hetzner/AWS/DO)                |
| `mobile` | Pinephone/Pinetab mobile device              |

## Project Structure Created

```
nixos-config/
├── .claude/
│   ├── CLAUDE.md                    # Project context (from device template)
│   ├── CODING-PRINCIPLES.md         # Rob Pike's 5 Rules + Linus Torvalds' principles
│   ├── TESTING.md                   # Testing guide for Rust + NixOS
│   ├── SECURITY.md                  # Security architecture and hardening checklist
│   ├── DEVELOPMENT.md               # Development workflow and common tasks
│   ├── ARCHITECTURE-PATTERNS.md     # NixOS module system, flake patterns, systemd
│   ├── PERFORMANCE.md               # Build performance, store management, service tuning
│   └── DATA-STRUCTURES.md           # Nix option types, attrsets, Rust types
├── flake.nix            # Flake inputs and outputs
├── flake.lock           # Locked dependencies
├── configuration.nix    # Main configuration
├── hardware-configuration.nix
├── modules/             # Custom modules
└── secrets/             # Encrypted secrets (agenix)
```

The agent must copy `templates/CLAUDE.md` (from the chosen device template)
**and all seven documentation files** into `.claude/` when initialising a project.
All seven files are referenced from CLAUDE.md and must travel with it:

- `templates/CODING-PRINCIPLES.md`      → `.claude/CODING-PRINCIPLES.md`
- `templates/TESTING.md`                → `.claude/TESTING.md`
- `templates/SECURITY.md`               → `.claude/SECURITY.md`
- `templates/DEVELOPMENT.md`            → `.claude/DEVELOPMENT.md`
- `templates/ARCHITECTURE-PATTERNS.md`  → `.claude/ARCHITECTURE-PATTERNS.md`
- `templates/PERFORMANCE.md`            → `.claude/PERFORMANCE.md`
- `templates/DATA-STRUCTURES.md`        → `.claude/DATA-STRUCTURES.md`

After copying, update the `[Insert Date]` placeholder in `SECURITY.md` and
`DEVELOPMENT.md` to today's date (DD/MM/YYYY).

## Required Information

If not provided, the agent will ask:

- **Template:** Which device type?
- **Hostname:** What to name the machine?
- **Hardware:** CPU, GPU, storage details
- **Profile:** minimal or full?

## Example References

The agent uses patterns from:

- `examples/nixos/flakes/BASIC-FLAKE.md` - Flake structure
- `examples/nixos/hardware/INTEL-LAPTOP.md` - Hardware configuration
- `examples/vault/secrets-injection/AGENIX-SECRETS.md` - Secrets setup

## Usage Examples

```
/syntek-infra:init
/syntek-infra:init laptop
/syntek-infra:init server
/syntek-infra:init router
```

## User's Request

$ARGUMENTS
