# SYNTEK-INFRA-PLUGIN ARCHITECTURAL PLAN

## Overview

The syntek-infra is a Claude Code plugin that provides NixOS configuration
management and Wireguard VPN setup capabilities across multiple device types and
network scenarios. It integrates with Hashicorp Vault for secrets management,
Vaultwarden for password storage, and provides Claude AI agents to automate
infrastructure provisioning.

## Requirements

### Functional Requirements

1. **NixOS Configuration Management**
   - Support 10 distinct device categories (workstations, mobile, network,
     servers)
   - Provide modular, reusable NixOS configurations
   - Integrate development tool configurations (Zed, Neovim, VS Code)
   - Support dockerised application stacks

2. **Wireguard VPN Management**
   - Outbound VPN via SSH + Mullvad (multi-proxy rotation)
   - VM access to client environments via Raspberry Pi
   - Remote access to home devices
   - Full network routing through home/business server

3. **Secrets Management Integration**
   - Hashicorp Vault for configuration variables
   - Vaultwarden for password storage
   - Secure credential injection into NixOS configs

4. **Claude Code Plugin Functionality**
   - Skills for NixOS config generation
   - Skills for Wireguard setup automation
   - Integration with syntek-dev-suite and syntek-rust-security plugins

### Non-Functional Requirements

1. **Security**: All secrets must be encrypted and never committed to git
2. **Modularity**: Configurations must be composable and reusable
3. **Idempotency**: Running configuration commands multiple times should be safe
4. **Documentation**: Every module must be self-documenting
5. **Testing**: Configurations should be testable in VM environments before
   deployment

## Technical Design

### Architecture Overview

This is a **Claude Code plugin** following the same structure as
syntek-dev-suite.

```
syntek-infra-plugin/
├── .claude-plugin/                  # Plugin metadata (Claude Code reads this)
│   ├── plugin.json                  # Name, version, author, keywords
│   ├── marketplace.json             # Marketplace registration
│   └── README.md                    # Folder documentation
├── agents/                          # Agent definitions (Markdown files)
│   ├── infra-architect.md           # High-level infrastructure planning
│   ├── nixos-builder.md             # NixOS configuration generation
│   ├── network-engineer.md          # Network and Wireguard specialist
│   ├── vault-manager.md             # Secrets management specialist
│   └── README.md                    # Agent reference documentation
├── commands/                        # Slash commands (Markdown files)
│   ├── nixos.md                     # /nixos command
│   ├── wireguard.md                 # /wireguard command
│   ├── secrets.md                   # /secrets command
│   ├── deploy.md                    # /deploy command
│   ├── init.md                      # /init command (project setup)
│   └── README.md                    # Command reference documentation
├── skills/                          # Stack-specific guidance
│   ├── global-workflow/             # Always applied
│   │   └── SKILL.md
│   ├── device-laptop/               # Laptop-specific guidance
│   │   └── SKILL.md
│   ├── device-server/               # Server-specific guidance
│   │   └── SKILL.md
│   ├── device-router/               # Router-specific guidance
│   │   └── SKILL.md
│   ├── device-mobile/               # Mobile-specific guidance
│   │   └── SKILL.md
│   └── README.md                    # Skill documentation
├── plugins/                         # Rust/Nix helper tools
│   ├── src/                         # Rust source code
│   │   ├── main.rs                  # CLI entry point
│   │   ├── nix.rs                   # Nix/Flakes detection
│   │   ├── nixos.rs                 # NixOS system helpers
│   │   ├── vault.rs                 # Hashicorp Vault integration
│   │   ├── vaultwarden.rs           # Vaultwarden integration
│   │   └── wireguard.rs             # Wireguard helpers
│   ├── Cargo.toml                   # Rust dependencies
│   ├── flake.nix                    # Nix flake for building
│   └── README.md                    # Tool reference documentation
├── templates/                       # Project initialisation templates
│   ├── laptop-workstation.md        # Developer laptop template
│   ├── home-server.md               # Home/business server template
│   ├── diy-router.md                # DIY router template
│   ├── cloud-server.md              # Cloud server template
│   ├── mobile-device.md             # Pinephone/Pinetab template
│   └── README.md                    # Template documentation
├── examples/                        # Code examples by domain
│   ├── nixos/                       # NixOS configuration examples
│   │   ├── hardware/                # Hardware detection examples
│   │   ├── services/                # Service configuration examples
│   │   ├── flakes/                  # Flake configuration examples
│   │   └── profiles/                # Profile composition examples
│   ├── wireguard/                   # Wireguard configuration examples
│   │   ├── outbound-vpn/            # Mullvad + SSH examples
│   │   ├── site-to-site/            # Client access examples
│   │   ├── road-warrior/            # Remote access examples
│   │   └── full-tunnel/             # Full routing examples
│   ├── vault/                       # Vault integration examples
│   │   ├── secrets-injection/       # Secret injection patterns
│   │   └── key-rotation/            # Key rotation examples
│   └── README.md                    # Examples documentation
├── nix-modules/                     # NixOS module library (actual Nix code)
│   ├── hardware/                    # Hardware-specific configs
│   │   ├── common.nix
│   │   ├── workstation/
│   │   ├── mobile/
│   │   ├── embedded/
│   │   └── cloud/
│   ├── profiles/                    # Device profile configs
│   │   ├── workstation/
│   │   ├── mobile/
│   │   ├── network/
│   │   └── cloud/
│   ├── services/                    # Service configurations
│   │   ├── docker/
│   │   ├── wireguard/
│   │   ├── vault/
│   │   ├── backup/
│   │   └── monitoring/
│   ├── development/                 # Development tool configs
│   │   ├── editors/
│   │   ├── languages/
│   │   ├── containers/
│   │   └── stacks/
│   └── networking/                  # Network and VPN configs
│       ├── firewall/
│       ├── dns/
│       ├── vpn/
│       └── routing/
├── docs/                            # Documentation
│   ├── PLANS/                       # Architecture plans
│   ├── GUIDES/                      # User guides
│   └── REFERENCE/                   # Technical reference
├── tests/                           # Testing infrastructure
│   ├── vm/                          # VM test configurations
│   └── integration/                 # Integration tests
├── config.json                      # Hooks and tool definitions
├── plugin.json                      # Plugin metadata (root level)
├── manifest.json                    # Plugin manifest
├── CLAUDE.md                        # Project context for Claude
├── README.md                        # Main documentation
├── CHANGELOG.md                     # Version history
└── VERSION                          # Semantic version number
```

### Module Structure

#### 1. Hardware Modules (`modules/hardware/`)

Device-specific configurations that handle hardware detection and driver setup.

```
hardware/
├── common.nix                       # Common hardware settings
├── workstation/
│   ├── desktop.nix                  # Desktop PC hardware
│   └── laptop.nix                   # Laptop hardware (power, battery)
├── mobile/
│   ├── pinephone.nix                # Pinephone hardware config
│   └── pinetab.nix                  # Pinetab hardware config
├── embedded/
│   ├── raspberry-pi.nix             # Raspberry Pi variants
│   └── router-hardware.nix          # DIY router hardware
└── cloud/
    ├── hetzner.nix                  # Hetzner cloud configs
    ├── aws.nix                      # AWS EC2 configs
    └── digitalocean.nix             # DigitalOcean configs
```

#### 2. Profile Modules (`modules/profiles/`)

High-level device profiles that combine hardware, services, and configurations.

```
profiles/
├── workstation/
│   ├── developer.nix                # Development workstation
│   └── minimal.nix                  # Minimal workstation
├── mobile/
│   ├── phone.nix                    # Phone profile
│   └── tablet.nix                   # Tablet profile
├── network/
│   ├── router.nix                   # DIY router profile
│   ├── nas.nix                      # NAS profile
│   ├── home-server.nix              # Home/business server
│   └── smart-home-hub.nix           # Smart home controller
└── cloud/
    ├── backup-server.nix            # Backblaze backup server
    ├── web-server.nix               # Web application server
    └── database-server.nix          # Database server
```

#### 3. Service Modules (`modules/services/`)

Service-specific configurations for applications and daemons.

```
services/
├── docker/
│   ├── default.nix                  # Docker daemon config
│   ├── compose.nix                  # Docker Compose integration
│   └── swarm.nix                    # Docker Swarm (optional)
├── wireguard/
│   ├── client.nix                   # Wireguard client config
│   ├── server.nix                   # Wireguard server config
│   ├── mullvad.nix                  # Mullvad VPN integration
│   └── multi-proxy.nix              # Multi-proxy rotation
├── backup/
│   ├── backblaze.nix                # Backblaze B2 backup
│   └── restic.nix                   # Restic backup tool
├── vault/
│   ├── client.nix                   # Vault client config
│   └── server.nix                   # Vault server (self-hosted)
└── monitoring/
    ├── prometheus.nix               # Prometheus monitoring
    └── grafana.nix                  # Grafana dashboards
```

#### 4. Development Modules (`modules/development/`)

Development tool and environment configurations.

```
development/
├── editors/
│   ├── zed.nix                      # Zed editor config
│   ├── neovim.nix                   # Neovim + Lua config
│   └── vscode.nix                   # VS Code config
├── languages/
│   ├── rust.nix                     # Rust toolchain
│   ├── nodejs.nix                   # Node.js/npm/pnpm
│   ├── python.nix                   # Python/pip/poetry
│   └── nix.nix                      # Nix development tools
├── containers/
│   ├── docker.nix                   # Docker development
│   └── podman.nix                   # Podman alternative
└── stacks/
    ├── django-stack.nix             # Django/Postgres/GraphQL
    ├── nextjs-stack.nix             # Next.js/React/Tailwind
    └── react-native-stack.nix       # React Native/Nativewind
```

#### 5. Networking Modules (`modules/networking/`)

Network configuration and security settings.

```
networking/
├── firewall/
│   ├── default.nix                  # Standard firewall rules
│   ├── router.nix                   # Router-specific rules
│   └── server.nix                   # Server-specific rules
├── dns/
│   ├── resolver.nix                 # DNS resolver config
│   └── server.nix                   # DNS server (Pi-hole, etc.)
├── vpn/
│   ├── wireguard/
│   │   ├── site-to-site.nix         # Site-to-site VPN
│   │   ├── road-warrior.nix         # Remote access VPN
│   │   └── mesh.nix                 # Mesh VPN network
│   └── ssh-tunnel.nix               # SSH tunneling config
└── routing/
    ├── nat.nix                      # NAT configuration
    └── policy-routing.nix           # Policy-based routing
```

### Wireguard Architecture

#### Use Case 1: Outbound VPN (SSH + Mullvad Multi-Proxy)

```
[Local Device] → [SSH Tunnel] → [VPS with Mullvad] → [Internet]
                                      ↓
                              [Proxy Rotation Script]
```

**Components:**

- Wireguard client on local device
- SSH tunnel to VPS
- Mullvad Wireguard config on VPS
- Proxy rotation service (systemd timer)
- Split tunneling rules for bypass scenarios

**Configuration Files:**

- `modules/networking/vpn/wireguard/outbound-vpn.nix`
- `templates/wireguard/mullvad-multi-proxy.conf`
- `plugins/wireguard-tool.py` (rotation logic)

#### Use Case 2: Client Environment VM Access

```
[Your Workstation] → [Wireguard] → [Client's Raspberry Pi] → [Client Network]
```

**Components:**

- Wireguard server on Raspberry Pi at client site
- Wireguard client on your workstation
- Firewall rules for specific access
- DNS integration for client network

**Configuration Files:**

- `modules/networking/vpn/wireguard/site-to-site.nix`
- `templates/wireguard/client-access.conf`

#### Use Case 3: Remote Home Access

```
[Your Remote Device] → [Wireguard] → [Home Router/Server] → [Home Devices]
```

**Components:**

- Wireguard server on home router/server
- Wireguard client on remote devices
- Dynamic DNS for home network
- Port forwarding on home router

**Configuration Files:**

- `modules/networking/vpn/wireguard/road-warrior.nix`
- `templates/wireguard/home-access.conf`

#### Use Case 4: Full Network Routing Through Home Server

```
[Any Device] → [Wireguard] → [Home/Business Server] → [Internet]
     ↓                              ↓
[All Traffic]              [Firewall/Filtering]
```

**Components:**

- Wireguard server on home/business server
- Wireguard client on all devices
- NAT and routing on server
- DNS filtering (optional: Pi-hole)
- Monitoring and logging

**Configuration Files:**

- `modules/networking/vpn/wireguard/full-tunnel.nix`
- `modules/networking/routing/nat.nix`
- `templates/wireguard/full-tunnel-server.conf`

### Integration Points

#### 1. Syntek-Dev-Suite Plugin Integration

**Shared Capabilities:**

- Git workflow management
- Documentation generation
- Testing infrastructure
- CI/CD pipeline setup

**Integration Method:**

- Import syntek-dev-suite skills via Claude Code skill system
- Reference syntek-dev-suite templates for dockerised services
- Use syntek-dev-suite logging patterns

**Files:**

- `skills/nixos-config/SKILL.md` (references dev-suite patterns)
- `modules/development/stacks/*.nix` (imports dev-suite configs)

#### 2. Syntek-Rust-Security Plugin Integration

**Shared Capabilities:**

- Rust security tool deployment
- Security auditing
- Cryptographic operations

**Integration Method:**

- NixOS modules for rust-security tools
- Docker container integration for security services
- Shared secrets via Vault

**Files:**

- `modules/services/security/rust-tools.nix`
- `profiles/cloud/security-server.nix`

#### 3. Hashicorp Vault Integration

**Purpose:**

- Store configuration variables
- Inject secrets into NixOS configs
- Rotate credentials automatically

**Implementation:**

```python
# plugins/vault-tool.py
class VaultClient:
    def get_wireguard_keys(self, device_name):
        """Retrieve Wireguard keys from Vault"""

    def store_nixos_secrets(self, hostname, secrets):
        """Store NixOS secrets in Vault"""

    def generate_config_with_secrets(self, template, vault_path):
        """Generate config with secrets injected from Vault"""
```

**Vault Path Structure:**

```
secret/
├── wireguard/
│   ├── devices/
│   │   ├── laptop-primary/
│   │   │   ├── private-key
│   │   │   └── public-key
│   │   └── phone/
│   │       ├── private-key
│   │       └── public-key
│   └── servers/
│       ├── home-server/
│       └── client-pi/
├── nixos/
│   ├── users/
│   │   └── passwords/
│   └── services/
│       ├── database/
│       └── api-keys/
└── development/
    ├── github-token
    └── docker-registry/
```

#### 4. Vaultwarden Integration

**Purpose:**

- Store user passwords
- Share passwords across devices
- Emergency access passwords

**Implementation:**

- Bitwarden CLI integration in NixOS
- Automatic password rotation
- Backup to Vaultwarden for critical system passwords

**Files:**

- `modules/services/vault/vaultwarden-client.nix`
- `plugins/vault-tool.py` (Vaultwarden API client)

### Configuration Management Approach

#### 1. Secrets Handling Strategy

**Never Commit Secrets to Git:**

```nix
# Good: Reference Vault secret
services.wireguard.privateKeyFile = "/run/secrets/wireguard-private-key";

# Bad: Hardcoded secret (NEVER DO THIS)
services.wireguard.privateKey = "ABCD1234...";
```

**Secret Injection Methods:**

1. **Build Time** (via Nix)

   ```nix
   # Use agenix or sops-nix for encrypted secrets in git
   age.secrets.wireguard-key = {
     file = ../secrets/wireguard-key.age;
     path = "/run/secrets/wireguard-private-key";
   };
   ```

2. **Runtime** (via Vault)

   ```bash
   # Systemd service fetches from Vault on startup
   ExecStartPre=/usr/bin/vault-fetch wireguard/private-key
   ```

3. **Deployment Time** (via deployment tool)
   ```python
   # Python plugin injects secrets during deployment
   vault_client.inject_secrets(config_path, device_name)
   ```

#### 2. Configuration Generation Workflow

```
1. User runs Claude skill: /syntek-infra:nixos-config --device laptop-primary
   ↓
2. Claude agent reads device requirements
   ↓
3. Agent selects appropriate modules (hardware/profiles/services)
   ↓
4. Agent generates configuration.nix from templates
   ↓
5. Plugin fetches secrets from Vault
   ↓
6. Plugin validates configuration (nixos-rebuild dry-run)
   ↓
7. Configuration saved to configs/workstations/laptop-primary/
   ↓
8. User reviews and deploys
```

#### 3. Configuration Versioning

**Git Strategy:**

- Configurations tracked in git (without secrets)
- Each device has a dedicated branch or directory
- Main branch contains templates and modules only

**Version Tags:**

```
configs/workstations/laptop-primary/
├── configuration.nix
├── hardware-configuration.nix
├── VERSION                          # Current version number
└── CHANGELOG.md                     # Changes per version
```

### Claude Code Plugin Structure

Following the syntek-dev-suite pattern, this plugin uses:

- **Agents** (Markdown files with instructions for specialised roles)
- **Commands** (Markdown files that invoke agents via slash commands)
- **Skills** (Device/stack-specific guidance loaded based on project type)
- **Plugins** (Rust CLI tools for environment detection and automation)
- **Templates** (Project initialisation files)
- **Examples** (Code snippets and Nix patterns)

**Note:** This is a Claude Code plugin only. It provides guidance and tooling
for Claude to assist users with NixOS and Wireguard configurations. The
`nix-modules/` directory contains reference examples for agents to use, not
production infrastructure.

#### Commands (Slash Commands)

**1. `/syntek-infra:nixos` Command**

**Purpose:** Generate NixOS configuration for a device

**Usage:**

```
/syntek-infra:nixos                           # Interactive mode
/syntek-infra:nixos laptop-primary developer  # Device + profile
/syntek-infra:nixos home-server --services docker,wireguard
```

**Command File:** `commands/nixos.md`

```markdown
---
description: '[Agent] Generate NixOS configuration'
usage: /syntek-infra:nixos
---

Spawn the `syntek-infra:nixos-builder` agent (model: sonnet)...

## Pre-flight: Run Plugin Tools

Before starting NixOS work, gather context using:

- python3 plugins/nix-tool.py detect
- python3 plugins/nixos-tool.py status
- python3 plugins/vault-tool.py status

**User's Request:** $ARGUMENTS
```

**2. `/syntek-infra:wireguard` Command**

**Purpose:** Configure Wireguard VPN for specific use case

**Usage:**

```
/syntek-infra:wireguard                       # Interactive mode
/syntek-infra:wireguard outbound-vpn          # Mullvad setup
/syntek-infra:wireguard home-access phone     # Road warrior config
```

**Command File:** `commands/wireguard.md`

**3. `/syntek-infra:secrets` Command**

**Purpose:** Manage secrets in Vault and Vaultwarden

**Usage:**

```
/syntek-infra:secrets                         # Interactive mode
/syntek-infra:secrets sync vault vaultwarden  # Sync between services
/syntek-infra:secrets fetch laptop-primary    # Fetch device secrets
```

**Command File:** `commands/secrets.md`

**4. `/syntek-infra:deploy` Command**

**Purpose:** Deploy NixOS configuration to target device

**Usage:**

```
/syntek-infra:deploy                          # Interactive mode
/syntek-infra:deploy laptop-primary local     # Local deployment
/syntek-infra:deploy home-server 192.168.1.10 # Remote deployment
```

**Command File:** `commands/deploy.md`

**5. `/syntek-infra:init` Command**

**Purpose:** Initialise a new NixOS configuration project

**Usage:**

```
/syntek-infra:init                            # Interactive mode
/syntek-infra:init laptop                     # Use laptop template
```

**Command File:** `commands/init.md`

#### Agents

**1. `infra-architect.md`**

```markdown
---
name: infra-architect
description: High-level infrastructure planning and deployment orchestration.
model: sonnet
---

# LOAD PROJECT CONTEXT (CRITICAL - DO THIS FIRST)

1. Read CLAUDE.md to understand the device type
2. Load relevant device skill (device-laptop, device-server, etc.)
3. Load global workflow skill
4. Run plugin tools to detect environment

# CAPABILITIES

- Understand user infrastructure requirements
- Design multi-device network topologies
- Orchestrate deployments across multiple devices
- Plan Wireguard VPN configurations
- Coordinate with Vault for secrets management

# REQUIRED INFORMATION

Ask if not in CLAUDE.md:

- Target device type (laptop/server/router/mobile/cloud)
- Network topology requirements
- Services to deploy
- Vault availability and credentials
```

**2. `nixos-builder.md`**

```markdown
---
name: nixos-builder
description: NixOS configuration generation and validation specialist.
model: sonnet
---

# CAPABILITIES

- Generate NixOS configurations using Flakes
- Compose modules from nix-modules/ library
- Validate configurations before deployment
- Debug NixOS build errors
- Optimise configurations for performance

# WORKFLOW

1. Detect current Nix environment (run nix-tool.py)
2. Identify target device and profile
3. Select appropriate modules
4. Generate flake.nix with inputs
5. Generate configuration.nix
6. Validate with `nix flake check`
7. Present configuration for user approval
```

**3. `network-engineer.md`**

```markdown
---
name: network-engineer
description: Network and Wireguard VPN configuration specialist.
model: sonnet
---

# CAPABILITIES

- Design Wireguard VPN topologies
- Configure outbound VPN (Mullvad + SSH rotation)
- Setup site-to-site VPNs (client access)
- Configure road warrior VPN (remote access)
- Setup full tunnel routing
- Configure firewall and routing rules

# WIREGUARD USE CASES

1. **outbound-vpn**: Mullvad + SSH multi-proxy rotation
2. **client-access**: VM into client via Raspberry Pi
3. **home-access**: Remote access to home devices
4. **full-tunnel**: Route all traffic through home server
```

**4. `vault-manager.md`**

```markdown
---
name: vault-manager
description: Secrets management with Hashicorp Vault and Vaultwarden.
model: sonnet
---

# CAPABILITIES

- Manage secrets in Hashicorp Vault
- Sync passwords with Vaultwarden
- Generate and rotate Wireguard keys
- Inject secrets into NixOS configurations
- Setup agenix/sops-nix for encrypted git secrets

# VAULT PATH STRUCTURE

secret/ ├── wireguard/devices/{device}/ ├── wireguard/servers/{server}/ ├──
nixos/users/passwords/ ├── nixos/services/{service}/ └── development/{tool}/
```

#### Skills (Device-Specific Guidance)

Skills are loaded automatically based on the `Skill Target` in CLAUDE.md.

**1. `global-workflow/SKILL.md`** (Always Applied)

```markdown
# Global Workflow - NixOS Infrastructure

## Language & Formatting

- British English spelling
- Date format: DD/MM/YYYY
- Time format: 24-hour clock (14:30)

## Nix Standards

- Use Nix Flakes for all configurations
- NixOS stable channel (not unstable)
- Pure Nix language only (no JSON/YAML)
- All secrets via Vault, never in git

## Security Requirements

- Wait for user approval before any deployment
- Validate all configurations in VM first
- Use agenix/sops-nix for git secrets
- Never hardcode secrets in Nix files
```

**2. `device-laptop/SKILL.md`**

```markdown
# Device: Developer Laptop

## Hardware Considerations

- Battery management and power profiles
- Wireless networking (WiFi, Bluetooth)
- Display scaling for HiDPI
- Touchpad gestures

## Development Environment

- Zed editor configuration (integrate zedconfig)
- Neovim + Lua configuration (when available)
- VS Code as backup
- Docker and container development

## Wireguard

- Outbound VPN via Mullvad (default)
- Home access VPN (optional)
```

**3. `device-server/SKILL.md`**

```markdown
# Device: Home/Business Server

## Services

- Docker daemon and Compose
- Wireguard server (full tunnel)
- Vault server (self-hosted)
- Backup services (Backblaze B2)
- Monitoring (Prometheus/Grafana)

## Network

- Static IP configuration
- Firewall rules for services
- NAT for client routing
- DNS server (optional Pi-hole)
```

#### Rust Plugin Tools

A single Rust CLI tool (`syntek-infra-tool`) with subcommands. Returns
structured JSON for agent consumption.

**Project Structure:**

```
plugins/
├── src/
│   ├── main.rs           # CLI entry point with clap
│   ├── nix.rs            # Nix/Flakes detection
│   ├── nixos.rs          # NixOS system helpers
│   ├── vault.rs          # Hashicorp Vault integration
│   ├── vaultwarden.rs    # Vaultwarden integration
│   └── wireguard.rs      # Wireguard helpers
├── Cargo.toml
├── flake.nix             # Build with Nix
└── README.md
```

**CLI Subcommands:**

```bash
# Nix detection
syntek-infra-tool nix detect          # Detect Nix version, flakes support
syntek-infra-tool nix channels        # List configured channels
syntek-infra-tool nix flake-check     # Validate a flake

# NixOS system
syntek-infra-tool nixos status        # Current generation and version
syntek-infra-tool nixos generations   # List generations for rollback
syntek-infra-tool nixos validate      # Validate configuration

# Vault
syntek-infra-tool vault status        # Check connectivity and auth
syntek-infra-tool vault read <path>   # Read secret
syntek-infra-tool vault write <path>  # Write secret

# Vaultwarden
syntek-infra-tool vaultwarden status  # Check connectivity
syntek-infra-tool vaultwarden sync    # Sync from Vault

# Wireguard
syntek-infra-tool wireguard keygen    # Generate keypair
syntek-infra-tool wireguard qr        # Generate QR code
```

**Cargo.toml Dependencies:**

```toml
[package]
name = "syntek-infra-tool"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
base64 = "0.22"
```

#### Templates (Project Initialisation)

Templates are copied to `.claude/CLAUDE.md` when initialising a new device
config.

**1. `laptop-workstation.md`**

```markdown
# NixOS Laptop Configuration

**Device:** [Insert Device Name] **Hostname:** [Insert Hostname] **Stack:**
NixOS + Flakes

## Skill Target
```

Skill Target: device-laptop

```

## Hardware
- CPU: [Intel/AMD]
- GPU: [Intel/NVIDIA/AMD]
- WiFi: [Chipset]
- Storage: [SSD/NVMe size]

## Profiles
- [x] Developer workstation
- [ ] Minimal

## Services
- [x] Docker
- [x] Wireguard (outbound-vpn)
- [ ] Vault client

## Development Tools
- [x] Zed editor
- [ ] Neovim + Lua
- [ ] VS Code

## Placeholders to Replace
- [Insert Device Name]
- [Insert Hostname]
```

**2. `home-server.md`** **3. `diy-router.md`** **4. `cloud-server.md`** **5.
`mobile-device.md`**

#### config.json (Tool Definitions)

```json
{
  "tools": [
    {
      "name": "nix_status",
      "description": "Detect Nix installation, version, and flakes support",
      "input_schema": {
        "type": "object",
        "properties": {}
      },
      "execution": {
        "command": "./plugins/syntek-infra-tool",
        "args": ["nix", "detect"]
      }
    },
    {
      "name": "nixos_status",
      "description": "Get current NixOS generation and system status",
      "input_schema": {
        "type": "object",
        "properties": {}
      },
      "execution": {
        "command": "./plugins/syntek-infra-tool",
        "args": ["nixos", "status"]
      }
    },
    {
      "name": "vault_status",
      "description": "Check Hashicorp Vault connectivity and auth",
      "input_schema": {
        "type": "object",
        "properties": {}
      },
      "execution": {
        "command": "./plugins/syntek-infra-tool",
        "args": ["vault", "status"]
      }
    },
    {
      "name": "wireguard_keygen",
      "description": "Generate Wireguard key pair",
      "input_schema": {
        "type": "object",
        "properties": {}
      },
      "execution": {
        "command": "./plugins/syntek-infra-tool",
        "args": ["wireguard", "keygen"]
      }
    }
  ]
}
```

## Implementation Phases

**Note:** These phases focus on building the Claude Code plugin itself, not the
actual NixOS infrastructure. The plugin provides agents, commands, skills,
examples, and tools that help Claude assist users with NixOS and Wireguard
configurations.

### Phase 1: Plugin Foundation

**Goal:** Establish Claude Code plugin structure and core files

**Tasks:**

- [ ] Create directory structure (agents/, commands/, skills/, plugins/,
      templates/, examples/)
- [ ] Set up `.claude-plugin/plugin.json` and `marketplace.json`
- [ ] Create `config.json` with tool definitions
- [ ] Create `manifest.json` and root `plugin.json`
- [ ] Write `CLAUDE.md` project context
- [ ] Create `README.md` with usage instructions
- [ ] Set up `VERSION` and `CHANGELOG.md`

**Deliverable:** Empty but valid Claude Code plugin structure that can be
installed

---

### Phase 2: Core Agents and Commands

**Goal:** Create the primary agents and slash commands

**Tasks:**

- [ ] Create `agents/infra-architect.md` agent
- [ ] Create `agents/nixos-builder.md` agent
- [ ] Create `agents/network-engineer.md` agent
- [ ] Create `agents/vault-manager.md` agent
- [ ] Create `commands/nixos.md` command
- [ ] Create `commands/wireguard.md` command
- [ ] Create `commands/secrets.md` command
- [ ] Create `commands/deploy.md` command
- [ ] Create `commands/init.md` command
- [ ] Create `agents/README.md` and `commands/README.md`

**Deliverable:** Working agents and commands that can be invoked via Claude Code

**Dependencies:** Phase 1

---

### Phase 3: Skills and Templates

**Goal:** Create device-specific skills and project templates

**Tasks:**

- [ ] Create `skills/global-workflow/SKILL.md`
- [ ] Create `skills/device-laptop/SKILL.md`
- [ ] Create `skills/device-server/SKILL.md`
- [ ] Create `skills/device-router/SKILL.md`
- [ ] Create `skills/device-mobile/SKILL.md`
- [ ] Create `templates/laptop-workstation.md`
- [ ] Create `templates/home-server.md`
- [ ] Create `templates/diy-router.md`
- [ ] Create `templates/cloud-server.md`
- [ ] Create `templates/mobile-device.md`
- [ ] Create `skills/README.md` and `templates/README.md`

**Deliverable:** Skills that provide context-aware guidance and templates for
project initialisation

**Dependencies:** Phase 2

---

### Phase 4: Rust Plugin Tools

**Goal:** Build the Rust CLI tool for environment detection

**Tasks:**

- [ ] Set up `plugins/Cargo.toml` and `plugins/flake.nix`
- [ ] Implement `plugins/src/main.rs` with clap CLI
- [ ] Implement `plugins/src/nix.rs` (Nix detection)
- [ ] Implement `plugins/src/nixos.rs` (NixOS helpers)
- [ ] Implement `plugins/src/vault.rs` (Vault integration)
- [ ] Implement `plugins/src/vaultwarden.rs` (Vaultwarden integration)
- [ ] Implement `plugins/src/wireguard.rs` (Wireguard helpers)
- [ ] Write tests for all modules
- [ ] Create `plugins/README.md`

**Deliverable:** Working `syntek-infra-tool` binary that agents can invoke

**Dependencies:** Phase 1

---

### Phase 5: Examples and Reference Nix Code

**Goal:** Create example code for agents to reference

**Tasks:**

- [ ] Create `examples/nixos/hardware/` examples
- [ ] Create `examples/nixos/services/` examples
- [ ] Create `examples/nixos/flakes/` examples
- [ ] Create `examples/nixos/profiles/` examples
- [ ] Create `examples/wireguard/outbound-vpn/` examples
- [ ] Create `examples/wireguard/site-to-site/` examples
- [ ] Create `examples/wireguard/road-warrior/` examples
- [ ] Create `examples/wireguard/full-tunnel/` examples
- [ ] Create `examples/vault/secrets-injection/` examples
- [ ] Create `examples/vault/key-rotation/` examples
- [ ] Create `nix-modules/` reference library (optional)
- [ ] Create `examples/README.md`

**Deliverable:** Comprehensive examples that agents can reference when helping
users

**Dependencies:** Phase 2, Phase 3

---

### Phase 6: Documentation and Testing

**Goal:** Complete documentation and test the plugin

**Tasks:**

- [ ] Create `docs/GUIDES/getting-started.md`
- [ ] Create `docs/GUIDES/laptop-setup.md`
- [ ] Create `docs/GUIDES/wireguard-setup.md`
- [ ] Create `docs/REFERENCE/agents.md`
- [ ] Create `docs/REFERENCE/commands.md`
- [ ] Create `docs/REFERENCE/tools.md`
- [ ] Test all commands work correctly
- [ ] Test agent responses are accurate
- [ ] Test Rust tools return valid JSON
- [ ] Update `README.md` with full documentation

**Deliverable:** Production-ready plugin with complete documentation

**Dependencies:** All previous phases

---

## Risks & Mitigations

| Risk                                        | Likelihood | Impact | Mitigation                                                                                     |
| ------------------------------------------- | ---------- | ------ | ---------------------------------------------------------------------------------------------- |
| **Claude Code plugin API changes**          | Low        | High   | Version lock plugin. Monitor Claude SDK updates. Follow syntek-dev-suite patterns.             |
| **Agent instructions become outdated**      | Medium     | Medium | Regular review of NixOS/Wireguard best practices. Version examples with NixOS releases.        |
| **Rust tool compilation issues**            | Low        | Medium | Use Nix flake for reproducible builds. Test on multiple platforms.                             |
| **Examples contain security issues**        | Low        | High   | Never include real secrets in examples. Review all examples for security best practices.       |
| **Integration with syntek-dev-suite fails** | Low        | Medium | Follow established plugin patterns. Regular integration testing.                               |
| **NixOS complexity overwhelms users**       | Medium     | Medium | Provide clear documentation. Start with simple laptop profile. Gradual complexity in examples. |
| **Incomplete device coverage**              | High       | Low    | Prioritise laptop first. Add other devices as hardware becomes available.                      |

## Open Questions - RESOLVED

### Technical Decisions - CONFIRMED

- [x] **NixOS vs. NixOS-unstable:** **NixOS stable** - Stability is prioritised
      over bleeding-edge packages.
- [x] **Flakes vs. Classic Nix:** **Nix Flakes** - Modern standard, better
      reproducibility and dependency management.
- [x] **Secret Management Tool:** **Hashicorp Vault** for configuration
      variables and secrets, **Vaultwarden** for user passwords. We will use
      agenix or sops-nix for encrypted secrets in git that reference Vault at
      runtime.
- [x] **Deployment Tool:** **nixos-rebuild** for simple local deployments,
      **deploy-rs** combined with custom Rust scripts for complex multi-device
      orchestration.
- [x] **Configuration Language:** **Pure Nix** for all NixOS configurations.
      Custom rebuild/deployment scripts may use Rust.

### User Experience - CONFIRMED

- [x] **Skill Naming:** **Short names** - `/nixos`, `/wireguard`, `/secrets`,
      `/deploy` (within the syntek-infra namespace)
- [x] **Configuration Review:** **Yes** - Agent must wait for user approval
      before deploying any configuration changes.
- [x] **Error Handling:** **Middle ground** - Technical enough to be useful,
      user-friendly enough to understand. Include both error codes and plain
      English explanations.
- [x] **Default Profiles:** **Both** - Provide `minimal` and `full` (batteries-
      included) profiles that users can select during configuration.

### Integration - CONFIRMED

- [x] **Zed Config Installer:** Located at
      `/home/sam-dev/Repos/personal/zedconfig/`. Structure includes: -
      `install.sh` - Main installer with environment detection
      (WSL/Linux/macOS) - `setup.sh` - Configuration setup - `verify-setup.sh` -
      Installation verification - `ssh-setup.sh` - SSH key configuration -
      `config/zed/` - Zed editor configuration files - `config/git/` - Git
      configuration files - `linters/` - Linter configurations - Supports apt,
      dnf, pacman, and brew package managers
- [x] **Neovim Config:** Expected to be built soon. Will integrate once
      available. Phase 5 will accommodate this.
- [x] **Syntek-Dev-Suite Integration:** **Ad-hoc** - Services will be dockerised
      based on project requirements rather than a fixed list.
- [x] **Syntek-Rust-Security Integration:** **Deferred** - Security tools will
      be determined during implementation based on actual needs.

### Infrastructure - CONFIRMED

- [x] **Vault Hosting:** **Self-hosted Hashicorp Vault** for configuration
      secrets and variables. **Self-hosted Vaultwarden** for password
      management.
- [x] **Backup Strategy:** **Backblaze B2** for all backups including Vault
      data.
- [x] **Test Environment:** **Laptop only** initially. Other hardware
      (Pinephone, Pinetab, Raspberry Pi, etc.) to be acquired later.
      Configurations will be built and VM-tested first.
- [x] **Cloud Resources:** **No budget currently**. Building configs in
      preparation for future SaaS deployment. Goal is to consolidate from
      scattered services (Google Workspace, Dashlane, AWS, etc.) to self-hosted
      infrastructure over the next 6 months.

### Prioritisation - CONFIRMED

- [x] **Device Priority:** **Laptop first**. All other hardware needs to be
      purchased. Focus on getting a solid workstation configuration working.
- [x] **Wireguard Use Case Priority:** **Outbound VPN with Mullvad** is primary.
      SSH access as secondary. Other use cases (home access, client access, full
      tunnel) will follow.
- [x] **Documentation Format:** **Markdown only**. No video tutorials.
- [x] **Testing Depth:** **Full TDD testing** with comprehensive manual guides.
      All configurations must be testable before deployment.

## Success Criteria

### MVP (Minimum Viable Product)

The Claude Code plugin is considered MVP-complete when:

1. **Plugin Structure:**
   - [ ] Valid plugin structure that installs in Claude Code
   - [ ] `.claude-plugin/plugin.json` and `marketplace.json` configured
   - [ ] `config.json` with tool definitions
   - [ ] `CLAUDE.md` and `README.md` complete

2. **Agents and Commands:**
   - [ ] All 4 agents defined and working
   - [ ] All 5 commands defined and working
   - [ ] Agents provide accurate NixOS guidance
   - [ ] Commands invoke correct agents

3. **Skills and Templates:**
   - [ ] Global workflow skill defined
   - [ ] At least laptop device skill defined
   - [ ] At least laptop template defined

4. **Documentation:**
   - [ ] Architecture plan (this document)
   - [ ] Basic README with usage instructions

### Production Ready

The plugin is considered production-ready when:

1. **Complete Plugin:**
   - [ ] All 4 agents fully defined with comprehensive instructions
   - [ ] All 5 commands working correctly
   - [ ] All 5 device skills defined
   - [ ] All 5 templates defined
   - [ ] Rust CLI tool (`syntek-infra-tool`) built and working

2. **Examples:**
   - [ ] NixOS examples (hardware, services, flakes, profiles)
   - [ ] Wireguard examples (all 4 use cases)
   - [ ] Vault examples (secrets injection, key rotation)

3. **Documentation:**
   - [ ] Complete user guides
   - [ ] Reference documentation for agents, commands, tools
   - [ ] Getting started guide

4. **Quality:**
   - [ ] All commands tested manually
   - [ ] Rust tools return valid JSON
   - [ ] No hardcoded paths or secrets

## Next Steps

### Immediate Actions

1. **Create Plugin Structure:**
   - Create all directories (agents/, commands/, skills/, plugins/, templates/,
     examples/)
   - Set up `.claude-plugin/` metadata files
   - Create empty `config.json`, `plugin.json`, `manifest.json`

2. **Begin Phase 1 Implementation:**
   - Write `CLAUDE.md` and `README.md`
   - Create `VERSION` and `CHANGELOG.md`
   - Test plugin installs in Claude Code

3. **Continue with Phase 2:**
   - Create agent markdown files
   - Create command markdown files
   - Test commands invoke agents correctly

### Handoff Signals

**After this plan is approved:**

- Run `/syntek-dev-suite:stories` to create user stories for Phase 1 tasks
- Run `/syntek-dev-suite:sprint` to organise stories into sprints
- Run `/syntek-dev-suite:setup` to initialise the plugin directory structure
- Run `/syntek-dev-suite:git` to setup git workflow for the plugin

**When ready to implement Rust tools:**

- Run `/syntek-dev-suite:backend` to implement Rust CLI tool
- Run `/syntek-dev-suite:test-writer` to create tests for Rust modules
- Run `/syntek-dev-suite:docs` to generate documentation

## Appendix

### Technology Stack Summary

**Core Technologies:**

- NixOS (Linux distribution)
- Nix (package manager and configuration language)
- Wireguard (VPN protocol)
- Hashicorp Vault (secrets management)
- Vaultwarden (password management)

**Development Tools:**

- Zed (primary editor)
- Neovim + Lua (primary editor - planned)
- VS Code (backup editor)

**Application Stacks:**

- Rust + Docker (security services)
- Django + Postgres + GraphQL + Docker (backend)
- Next.js + Node.js + React + TypeScript + Tailwind + Docker (web)
- React Native + TypeScript + Nativewind (mobile)

**Infrastructure:**

- Docker (containerisation)
- Cloud Providers: Hetzner, AWS, DigitalOcean
- Backup: Backblaze B2
- Hardware: x86_64 PCs/Laptops, ARM (Pinephone/Pinetab/Raspberry Pi)

### Reference Links

**NixOS Documentation:**

- NixOS Manual: https://nixos.org/manual/nixos/stable/
- Nix Package Search: https://search.nixos.org/packages
- NixOS Wiki: https://nixos.wiki/

**Wireguard Documentation:**

- Wireguard Quick Start: https://www.wireguard.com/quickstart/
- Wireguard on NixOS: https://nixos.wiki/wiki/WireGuard

**Secret Management:**

- agenix: https://github.com/ryantm/agenix
- sops-nix: https://github.com/Mic92/sops-nix
- Hashicorp Vault: https://www.vaultproject.io/docs

**Claude Code Plugin Development:**

- (Add links to syntek-dev-suite documentation once available)
