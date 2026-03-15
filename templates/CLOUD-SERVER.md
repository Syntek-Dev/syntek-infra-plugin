# NixOS Cloud Server Configuration

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

**Device:** [Cloud Provider - Instance Type] **Hostname:** [Insert Hostname]
**Stack:** NixOS + Flakes **Created:** [Insert Date]

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

## Cloud Provider

Select one:

- [ ] Hetzner Cloud
- [ ] AWS EC2
- [ ] DigitalOcean
- [ ] Other: [Specify]

## Instance Details

- **Region:** [e.g., eu-central-1, fsn1]
- **Instance Type:** [e.g., cx21, t3.micro]
- **vCPUs:** [Number]
- **RAM:** [Amount in GB]
- **Storage:** [Size and type]
- **IPv4:** [Public IP or "DHCP"]
- **IPv6:** [If enabled]

## Server Purpose

Select primary purpose:

- [ ] **Backup Server** - Backblaze B2 backup destination
- [ ] **Web Server** - Website and application hosting
- [ ] **Database Server** - PostgreSQL/MySQL hosting
- [ ] **VPN Endpoint** - Mullvad proxy / SSH jump host

## Services

### For Backup Server

- [x] Restic server
- [x] Backblaze B2 sync
- [x] SSH (key-only)
- [x] Firewall

### For Web Server

- [x] Nginx
- [x] Docker
- [x] Let's Encrypt (ACME)
- [x] PostgreSQL
- [ ] Redis

### For VPN Endpoint

- [x] Wireguard
- [x] SSH tunnel support
- [ ] Mullvad integration

## Network Configuration

- **Public IP:** [From provider]
- **Private Network:** [If VPC enabled]
- **Firewall:** Cloud provider + NixOS

## Firewall Rules

### Inbound

| Port  | Protocol | Source      | Purpose     |
| ----- | -------- | ----------- | ----------- |
| 22    | TCP      | Trusted IPs | SSH         |
| 80    | TCP      | Any         | HTTP (ACME) |
| 443   | TCP      | Any         | HTTPS       |
| 51820 | UDP      | Any         | Wireguard   |

## Secrets Management

Cloud servers should fetch secrets from Vault at runtime:

- **Vault Address:** [Home Vault or Cloud Vault]
- **Auth Method:** AppRole (recommended for servers)

## Backup Configuration

### If Backup Server

- **Restic Repository:** /var/lib/restic
- **Sync to B2:** Daily

### If Other Server Type

- **Backup to:** [Home server / Other cloud]
- **Schedule:** Daily

## Deployment

Cloud servers typically use:

```bash
# Remote deployment
nixos-rebuild switch --flake .#hostname --target-host root@IP
```

Or with deploy-rs for more complex setups.

## Provider-Specific Notes

### Hetzner

```nix
# Hetzner cloud-init compatibility
imports = [
  (modulesPath + "/profiles/qemu-guest.nix")
];
```

### AWS

```nix
# AWS EC2 compatibility
imports = [
  (modulesPath + "/virtualisation/amazon-image.nix")
];
```

### DigitalOcean

```nix
# DigitalOcean compatibility
imports = [
  (modulesPath + "/virtualisation/digital-ocean-config.nix")
];
```

## Placeholders to Replace

When using this template, replace:

- `[Cloud Provider - Instance Type]` - e.g., "Hetzner Cloud - CX21"
- `[Insert Hostname]` - e.g., "backup-server"
- `[Insert Date]` - e.g., "22/01/2026"
- Instance and region details
- IP addresses
- Firewall rules
