# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.0] - 24/02/2026

### Added

- `paths.rs` module for device-independent plugin root discovery
  - Uses `std::env::current_exe()` to walk up the directory tree
  - Locates plugin root by finding `.claude-plugin/plugin.json` marker
  - `SYNTEK_PLUGIN_DIR` env var as explicit override for edge cases
  - `syntek-infra-tool paths all` — returns JSON of all plugin directory paths
  - `syntek-infra-tool paths get <name>` — returns a single named path

- `docs.rs` module wired into CLI for project documentation management
  - `syntek-infra-tool docs list` — lists `.claude/` documentation files and presence
  - `syntek-infra-tool docs show <name>` — prints full content of a named doc file

- Four documentation templates now distributed with the plugin
  - `templates/CODING-PRINCIPLES.md` — Rob Pike's 5 Rules and Linus Torvalds' principles
  - `templates/TESTING.md` — Testing guide for Rust and NixOS
  - `templates/SECURITY.md` — Security architecture and secrets management checklist
  - `templates/DEVELOPMENT.md` — Development workflow and common tasks

- Required documentation section in `CLAUDE.md` — agents must copy all four files into `.claude/` on init
- `agents/infra-architect.md` updated with documentation copy workflow
- `commands/init.md` updated with documentation file requirements
- All device templates updated with documentation file references

### Fixed

- `hyprland.rs` now uses `anyhow::Result` consistently with all other modules
  — eliminates `Box<dyn std::error::Error>` type mismatch in `main.rs`

### Changed

- `syntek-infra-tool` bumped to `0.2.0` for new subcommand groups (`docs`, `paths`)
- Plugin version synced across `VERSION`, `plugin.json`, and `Cargo.toml`

## [1.0.0] - 24/01/2026

### Added

- Plugin metadata and configuration
  - .claude-plugin/plugin.json for Claude Code registration
  - .claude-plugin/marketplace.json for marketplace listing
  - config.json with Rust tool definitions

- Four infrastructure agents
  - infra-architect: High-level planning and deployment orchestration
  - nixos-builder: NixOS configuration generation and validation
  - network-engineer: Wireguard VPN and network security
  - vault-manager: Secrets management with Vault and Vaultwarden

- Five slash commands
  - /syntek-infra:init - Initialise new NixOS project
  - /syntek-infra:nixos - Generate NixOS configuration
  - /syntek-infra:wireguard - Configure Wireguard VPN
  - /syntek-infra:secrets - Manage secrets
  - /syntek-infra:deploy - Deploy configuration

- Device-specific skills
  - global-workflow: Standards for all configurations
  - device-laptop: Developer workstation guidance
  - device-server: Home/business server guidance
  - device-router: DIY router guidance
  - device-mobile: Pinephone/Pinetab guidance

- Project templates
  - laptop-workstation, home-server, diy-router
  - cloud-server, mobile-device

- Rust CLI tool (syntek-infra-tool)
  - Nix environment detection
  - NixOS system status
  - Wireguard key generation and QR codes
  - Vault and Vaultwarden integration

- Comprehensive examples
  - NixOS: flakes, modules, hardware (Intel, AMD, headless)
  - NixOS: nftables, systemd, ZFS, VLANs
  - Wireguard: Mullvad client, full tunnel server
  - Secrets: agenix and sops-nix integration

## [0.1.0] - 22/01/2026

### Added

- Initial project structure and README
