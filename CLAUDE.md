# Project Context for Claude Code

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

**Project Name:** Syntek Infra Plugin **Stack:** Claude Code Plugin (Rust + Nix)
**Description:** NixOS configuration, Hyprland compositor, and Wireguard VPN
setup assistant **Language:** British English (en_GB) **Timezone:**
Europe/London

## Project Type

This is a **Claude Code plugin** that provides agents, commands, skills, and
tools to help users with:

- NixOS configuration using Flakes
- Hyprland wayland compositor configuration
- Wireguard VPN setup (Mullvad, site-to-site, road warrior, full tunnel)
- Hashicorp Vault integration for secrets
- Vaultwarden integration for passwords

## Skill Target

```
Skill Target: global-workflow
```

Device-specific skills are loaded based on the target device type:

- `device-laptop` - Developer laptop configurations
- `device-server` - Home/business server configurations
- `device-router` - DIY router configurations
- `device-mobile` - Pinephone/Pinetab configurations

## Plugin Structure

```
syntek-infra-plugin/
├── .claude-plugin/      # Plugin metadata
├── agents/              # Agent definitions (Markdown)
├── commands/            # Slash commands (Markdown)
├── skills/              # Device-specific guidance
├── plugins/             # Rust CLI tool (syntek-infra-tool)
├── templates/           # Project initialisation templates
├── examples/            # NixOS, Wireguard, Vault examples
└── docs/                # Documentation
```

## Commands

- `/syntek-infra:nixos` - Generate NixOS configuration
- `/syntek-infra:hyprland` - Configure Hyprland compositor
- `/syntek-infra:wireguard` - Configure Wireguard VPN
- `/syntek-infra:secrets` - Manage secrets in Vault/Vaultwarden
- `/syntek-infra:deploy` - Deploy NixOS configuration
- `/syntek-infra:init` - Initialise a new NixOS project

## Technical Decisions

- **NixOS stable** (not unstable) for stability
- **Nix Flakes** for modern configuration management
- **Pure Nix** language only (no JSON/YAML configs)
- **Hashicorp Vault** for configuration secrets
- **Vaultwarden** for password management
- **nixos-rebuild** for simple deployments
- **deploy-rs** with Rust scripts for complex deployments

## Development Standards

- British English spelling throughout
- Date format: DD/MM/YYYY
- Time format: 24-hour clock (14:30)
- All secrets via Vault, never in git
- Wait for user approval before any deployment
- Validate configurations in VM before deployment

## Required Documentation

All projects initialised by this plugin must include these seven files in `.claude/`:

- **[CODING-PRINCIPLES.md](templates/CODING-PRINCIPLES.md)** — Rob Pike's 5 Rules, Linus Torvalds' principles, naming conventions, error handling, and the code review checklist. Read before writing or reviewing any code.
- **[TESTING.md](templates/TESTING.md)** — Testing guide for Rust and NixOS: unit tests, integration tests, `nixosTest` VM tests, property-based tests, and mocking patterns.
- **[SECURITY.md](templates/SECURITY.md)** — Security architecture: secrets management (Vault + agenix), Wireguard key handling, NixOS module hardening, SSH access control, and the security checklist.
- **[DEVELOPMENT.md](templates/DEVELOPMENT.md)** — Development workflow: getting started, the local development loop, deployment procedures, common tasks, and troubleshooting.
- **[ARCHITECTURE-PATTERNS.md](templates/ARCHITECTURE-PATTERNS.md)** — NixOS module system design, flake structure patterns, device profile pattern, systemd service patterns, secret injection architecture, Wireguard topologies, and Hyprland configuration patterns.
- **[PERFORMANCE.md](templates/PERFORMANCE.md)** — Nix build performance, binary caches, evaluation performance, Nix store management, systemd service tuning, and monitoring.
- **[DATA-STRUCTURES.md](templates/DATA-STRUCTURES.md)** — Nix type system, NixOS option type design, Rust newtypes and error types, configuration data modelling, and anti-patterns.

The `/syntek-infra:init` command copies all seven files into `.claude/` automatically alongside CLAUDE.md.

## Related Plugins

- `syntek-dev-suite` - Development workflow tooling
- `syntek-rust-security` - Rust security tooling
