# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
