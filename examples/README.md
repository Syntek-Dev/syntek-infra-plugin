# Examples

This directory contains example code that agents reference when helping users
with NixOS and Wireguard configurations.

## Directory Structure

```
examples/
├── nixos/
│   ├── flakes/
│   │   └── BASIC-FLAKE.md          # Flake structure and inputs
│   ├── modules/
│   │   └── CUSTOM-MODULE.md        # Custom NixOS module patterns
│   ├── hardware/
│   │   ├── INTEL-LAPTOP.md         # Intel CPU + integrated graphics
│   │   ├── AMD-LAPTOP.md           # AMD Ryzen + integrated Radeon
│   │   ├── AMD-GPU.md              # AMD discrete GPU configuration
│   │   └── HEADLESS-SERVER.md      # Server without GPU
│   ├── firewall/
│   │   └── NFTABLES-FIREWALL.md    # nftables firewall rules
│   ├── systemd/
│   │   └── SYSTEMD-SERVICES.md     # systemd service deployment
│   ├── filesystems/
│   │   └── ZFS-CONFIGURATION.md    # ZFS setup and management
│   └── networking/
│       └── VLAN-SEGMENTATION.md    # Network segmentation with VLANs
├── wireguard/
│   ├── outbound-vpn/
│   │   └── MULLVAD-CLIENT.md       # Mullvad + SSH proxy rotation
│   └── full-tunnel/
│       └── SERVER-CONFIG.md        # Route all traffic through server
└── vault/
    ├── secrets-injection/
    │   └── AGENIX-SECRETS.md       # agenix encrypted secrets
    └── sops-nix/
        └── SOPS-SECRETS.md         # sops-nix encrypted secrets
```

## Coverage

| Topic                        | Example File                                                                         |
| ---------------------------- | ------------------------------------------------------------------------------------ |
| NixOS modules & flakes       | `nixos/flakes/BASIC-FLAKE.md`, `nixos/modules/CUSTOM-MODULE.md`                      |
| WireGuard/VPN configuration  | `wireguard/outbound-vpn/MULLVAD-CLIENT.md`, `wireguard/full-tunnel/SERVER-CONFIG.md` |
| nftables/iptables rules      | `nixos/firewall/NFTABLES-FIREWALL.md`                                                |
| systemd service deployment   | `nixos/systemd/SYSTEMD-SERVICES.md`                                                  |
| ZFS/filesystem configuration | `nixos/filesystems/ZFS-CONFIGURATION.md`                                             |
| Network segmentation         | `nixos/networking/VLAN-SEGMENTATION.md`                                              |
| agenix secrets               | `vault/secrets-injection/AGENIX-SECRETS.md`                                          |
| sops-nix secrets             | `vault/sops-nix/SOPS-SECRETS.md`                                                     |
| Intel hardware               | `nixos/hardware/INTEL-LAPTOP.md`                                                     |
| AMD CPU hardware             | `nixos/hardware/AMD-LAPTOP.md`                                                       |
| AMD GPU hardware             | `nixos/hardware/AMD-GPU.md`                                                          |
| Headless servers             | `nixos/hardware/HEADLESS-SERVER.md`                                                  |

## How Examples Are Used

Agents reference these examples when:

1. Generating new configurations
2. Explaining patterns to users
3. Debugging configuration issues
4. Suggesting best practices

## Example Format

Each example file contains:

- **Title and description**
- **Use case explanation**
- **Complete, working code**
- **Comments explaining key points**
- **Variations for different scenarios**

## Contributing Examples

When adding new examples:

1. Use complete, working code (not snippets)
2. Include comments explaining decisions
3. Test the code works on NixOS stable
4. Follow the global-workflow standards (British English, Nix Flakes, pure Nix)
