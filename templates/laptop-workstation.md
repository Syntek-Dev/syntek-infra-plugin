# NixOS Laptop Configuration

**Device:** [Insert Device Name] **Hostname:** [Insert Hostname] **Stack:**
NixOS + Flakes **Created:** [Insert Date]

## Required Documentation

All code in this project follows the standards in these four files (in `.claude/`):

- **[CODING-PRINCIPLES.md](CODING-PRINCIPLES.md)** — Rob Pike's 5 Rules, Linus Torvalds' principles, naming conventions, error handling, and code review checklist.
- **[TESTING.md](TESTING.md)** — Testing guide for Rust and NixOS: unit, integration, nixosTest VM tests, and property-based tests.
- **[SECURITY.md](SECURITY.md)** — Security architecture: Vault + agenix secrets, Wireguard key handling, NixOS module hardening, and the security checklist.
- **[DEVELOPMENT.md](DEVELOPMENT.md)** — Development workflow: local dev loop, deployment procedures, common tasks, and troubleshooting.

Read all four before writing or reviewing any code.

## Skill Target

```
Skill Target: device-laptop
```

## Hardware

- **CPU:** [Intel/AMD - specify model]
- **GPU:** [Intel/NVIDIA/AMD - specify model]
- **RAM:** [Amount in GB]
- **Storage:** [SSD/NVMe - size]
- **WiFi:** [Chipset if known]
- **Display:** [Resolution and scaling needs]

## Profile

Select one:

- [ ] **Minimal** - Basic laptop with networking and shell
- [x] **Full** - Complete developer workstation

## Services

- [x] Docker
- [x] Wireguard (outbound-vpn via Mullvad)
- [ ] Wireguard (home-access)
- [ ] Vault client

## Development Tools

### Editors

- [x] Zed editor
- [ ] Neovim + Lua
- [ ] VS Code (backup)

### Languages

- [x] Rust (rustup)
- [x] Node.js 20
- [x] Python 3

### Utilities

- [x] Git + GitHub CLI
- [x] ripgrep, fd, jq
- [x] Docker Compose

## Secrets Management

- **Method:** [agenix / sops-nix / runtime-vault]
- **Vault Address:** [If using Vault]

## Network Configuration

- **WiFi:** NetworkManager
- **VPN:** Mullvad via Wireguard
- **Firewall:** Enabled, minimal ports

## Placeholders to Replace

When using this template, replace:

- `[Insert Device Name]` - e.g., "ThinkPad X1 Carbon Gen 11"
- `[Insert Hostname]` - e.g., "laptop-primary"
- `[Insert Date]` - e.g., "22/01/2026"
- Hardware specifications
- Select appropriate checkboxes
