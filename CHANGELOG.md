# Changelog

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## Table of Contents

- [Unreleased](#unreleased)
- [2.0.0 - 15/03/2026](#200---15032026)
- [0.1.0 - 22/01/2026](#010---22012026)

---

## [Unreleased]

### Added
- Nothing yet

---

## [2.0.0] - 15/03/2026

**First stable production release — corrected version number.** A prior
incorrect version bump set the version to `1.0.0` despite the project having
already reached `1.2.0`. This entry corrects that regression. The true version
progression is: `0.1.0` → `1.2.0` → **`2.0.0`** (this release). Version
`2.0.0` is designated as the first stable "going live" production milestone.

This release introduces a comprehensive seven-file `.claude/` documentation
set, dynamic reference loading across all agents and commands, full Hyprland
compositor support, dynamic plugin path resolution, a documentation CLI, and
breaking changes to plugin structure (template files renamed to UPPERCASE,
documentation set expanded from four files to seven).

### Added

- **Three new documentation templates** distributed to all initialised projects:
  - `templates/ARCHITECTURE-PATTERNS.md` — NixOS module system design, flake
    structure patterns, device profile pattern, systemd service hardening
    patterns, secret injection architecture, Wireguard topologies, and
    Hyprland configuration patterns
  - `templates/PERFORMANCE.md` — Nix build performance, binary caches, IFD
    avoidance, Nix store garbage collection, systemd service tuning, and
    performance monitoring for Wireguard, Vault, and Hyprland
  - `templates/DATA-STRUCTURES.md` — Nix type system, NixOS option type design,
    Rust newtypes, enums, and error types, configuration data modelling, and
    anti-patterns

- `templates/CLAUDE-MD-TEMPLATE.md` — canonical CLAUDE.md template for projects
  initialised by this plugin, listing all seven reference documents

- Dynamic `.claude/` reference loading added to all five agents; each agent now
  reads the relevant reference documents from the user's project at the start of
  every session:
  - `agents/infra-architect.md` — reads all seven reference docs
  - `agents/nixos-builder.md` — reads CODING-PRINCIPLES, SECURITY,
    ARCHITECTURE-PATTERNS, PERFORMANCE, DATA-STRUCTURES, TESTING
  - `agents/network-engineer.md` — reads CODING-PRINCIPLES, SECURITY,
    ARCHITECTURE-PATTERNS, DATA-STRUCTURES
  - `agents/vault-manager.md` — reads CODING-PRINCIPLES, SECURITY,
    ARCHITECTURE-PATTERNS
  - `agents/hyprland-configurator.md` — reads CODING-PRINCIPLES,
    ARCHITECTURE-PATTERNS, PERFORMANCE, DATA-STRUCTURES

- "Reference Documents" section added to all six commands, instructing the
  spawned agent which `.claude/` files to load:
  - `commands/nixos.md`, `commands/wireguard.md`, `commands/secrets.md`,
    `commands/hyprland.md`, `commands/deploy.md`, `commands/init.md`

- Dynamic plugin path resolution (`paths.rs`) for device-independent root
  discovery via `std::env::current_exe()` directory walk
  - `SYNTEK_PLUGIN_DIR` environment variable override for edge cases
  - `syntek-infra-tool paths all` — JSON output of all plugin directory paths
  - `syntek-infra-tool paths get <name>` — single named path lookup

- Documentation CLI (`docs.rs`) wired into `syntek-infra-tool`
  - `syntek-infra-tool docs list` — lists `.claude/` documentation files and
    their presence
  - `syntek-infra-tool docs show <name>` — prints full content of a named doc

- Comprehensive Hyprland compositor support
  - `agents/hyprland-configurator.md` — dedicated Hyprland agent
  - `commands/hyprland.md` — `/syntek-infra:hyprland` slash command
  - Examples: `BASIC-HYPRLAND.md`, `LAPTOP-HYPRLAND.md`, `MULTI-MONITOR.md`,
    `NVIDIA-HYPRLAND.md`, `AMD-HYPRLAND.md`, `HYPRLAND-NIXOS.md`

- `commands/init.md` and `agents/infra-architect.md` copy list expanded from
  four to seven template files; verbiage updated from "all four documentation
  files" to "all seven documentation files"

### Changed

- Documentation set for initialised projects expanded from four files to seven:
  added ARCHITECTURE-PATTERNS.md, PERFORMANCE.md, and DATA-STRUCTURES.md
  alongside the existing CODING-PRINCIPLES.md, TESTING.md, SECURITY.md, and
  DEVELOPMENT.md (breaking change for plugin consumers)
- All device and project template files renamed from lowercase to UPPERCASE for
  consistency: `LAPTOP-WORKSTATION.md`, `HOME-SERVER.md`, `DIY-ROUTER.md`,
  `CLOUD-SERVER.md`, `MOBILE-DEVICE.md`, `HYPRLAND-DESKTOP.md`,
  `HYPRLAND-LAPTOP.md` (breaking change in plugin structure)
- `templates/SECURITY.md`, `templates/DEVELOPMENT.md`, and
  `templates/CODING-PRINCIPLES.md` updated with proper version/metadata headers
- Plugin version aligned across `VERSION`, `plugin.json`, and `Cargo.toml`
- `syntek-infra-tool` internal version bumped for new subcommand groups
  (`docs`, `paths`)
- All Markdown documentation templates updated with consistent metadata headers

### Fixed

- `hyprland.rs` now uses `anyhow::Result` consistently, eliminating
  `Box<dyn std::error::Error>` type mismatch in `main.rs`
- Resolved clippy warnings in NixOS and Vaultwarden modules
- Removed unused variable in `vaultwarden` sync handler
- Corrected version regression: version incorrectly set to `1.0.0` despite
  prior history reaching `1.2.0`; corrected to `2.0.0`

### Removed

- Duplicate `templates/coding-principles.md` (lowercase) removed; the canonical
  file is now `templates/CODING-PRINCIPLES.md` (uppercase)

---

## [0.1.0] - 22/01/2026

### Added

- Initial project structure and README
