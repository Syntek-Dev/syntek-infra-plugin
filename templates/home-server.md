# NixOS Home/Business Server Configuration

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
Skill Target: device-server
```

## Hardware

- **CPU:** [Specify model]
- **RAM:** [Amount in GB]
- **Storage:** [Drives and RAID configuration]
- **Network:** [Number of NICs]

## Profile

Select one:

- [ ] **Minimal** - Basic server with SSH and Docker
- [x] **Full** - Complete home server with all services

## Server Role

This server will act as:

- [x] Wireguard VPN hub (full tunnel)
- [x] Docker host
- [x] Hashicorp Vault server
- [x] Backup destination
- [x] Monitoring hub
- [ ] DNS server (Pi-hole/AdGuard)
- [ ] Smart home controller

## Services

### Core

- [x] Docker + Docker Compose
- [x] SSH (key-only)
- [x] Firewall

### Wireguard VPN

- [x] VPN server for all devices
- [ ] Full tunnel routing
- [ ] Split tunnel routing

### Secrets

- [x] Hashicorp Vault (self-hosted)
- [ ] Vaultwarden (self-hosted)

### Backup

- [x] Restic
- [x] Backblaze B2 destination

### Monitoring

- [x] Prometheus
- [x] Grafana
- [x] Node exporter

## Network Configuration

- **IP:** [Static IP address]
- **Gateway:** [Router IP]
- **DNS:** [DNS servers]

## VPN Clients

Devices that will connect to this server:

| Device         | IP Address | Public Key Location            |
| -------------- | ---------- | ------------------------------ |
| laptop-primary | 10.100.0.2 | vault:wireguard/devices/laptop |
| phone          | 10.100.0.3 | vault:wireguard/devices/phone  |
| tablet         | 10.100.0.4 | vault:wireguard/devices/tablet |

## Secrets Management

- **Vault Address:** https://vault.home.lan:8200
- **Vault Storage:** File backend at /var/lib/vault

## Backup Configuration

- **Repository:** b2:bucket-name:/backups
- **Schedule:** Daily at 04:00
- **Paths:** /home, /var/lib, /etc/nixos

## Placeholders to Replace

When using this template, replace:

- `[Insert Device Name]` - e.g., "Custom Build Server"
- `[Insert Hostname]` - e.g., "home-server"
- `[Insert Date]` - e.g., "22/01/2026"
- Hardware specifications
- Network configuration
- VPN client details
