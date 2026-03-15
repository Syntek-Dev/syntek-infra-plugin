# Architecture Patterns

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Infrastructure Team
**Language:** British English (en_GB)
**Timezone:** Europe/London
**Plugin Scope:** syntek-infra (NixOS, Hyprland, Wireguard, Vault, Vaultwarden)

---

## Table of Contents

- [Overview](#overview)
- [NixOS Module System](#nixos-module-system)
  - [Module Structure](#module-structure)
  - [Option Declaration](#option-declaration)
  - [Conditional Configuration](#conditional-configuration)
  - [Module Assertions](#module-assertions)
- [Flake Structure Patterns](#flake-structure-patterns)
  - [Standard Flake Layout](#standard-flake-layout)
  - [Device Profile Pattern](#device-profile-pattern)
  - [Shared Module Library](#shared-module-library)
- [Systemd Service Patterns](#systemd-service-patterns)
  - [Service with Secret Injection](#service-with-secret-injection)
  - [Service Hardening Template](#service-hardening-template)
  - [Service Ordering and Dependencies](#service-ordering-and-dependencies)
- [Secret Injection Architecture](#secret-injection-architecture)
- [Wireguard Architecture Patterns](#wireguard-architecture-patterns)
  - [Hub-and-Spoke Pattern](#hub-and-spoke-pattern)
  - [Full Tunnel Pattern](#full-tunnel-pattern)
  - [Site-to-Site Pattern](#site-to-site-pattern)
- [Overlay Patterns](#overlay-patterns)
- [Cross-System Builds](#cross-system-builds)
- [Hyprland Configuration Patterns](#hyprland-configuration-patterns)
- [Architecture Checklist](#architecture-checklist)

---

## Overview

This guide covers architectural patterns for NixOS configurations managed by the syntek-infra plugin. It covers the module system, flake structure, systemd service patterns, and common infrastructure topologies.

Architecture in NixOS is declarative: you describe the desired state and the system converges to it. This changes how patterns work compared to imperative infrastructure tools. The patterns here leverage NixOS's strengths: reproducibility, atomic upgrades, and first-class module composition.

---

## NixOS Module System

### Module Structure

Every NixOS module follows the same structure. Import, declare options, and configure based on those options:

```nix
{ config, lib, pkgs, ... }:

{
  # 1. Imports — other modules this one depends on
  imports = [ ];

  # 2. Options — what this module exposes to other modules
  options.myService = {
    enable = lib.mkEnableOption "my service";
    port = lib.mkOption {
      type = lib.types.port;
      default = 8080;
      description = lib.mdDoc "Port the service listens on.";
    };
  };

  # 3. Config — what this module does when enabled
  config = lib.mkIf config.myService.enable {
    systemd.services.my-service = {
      description = "My Service";
      wantedBy = [ "multi-user.target" ];
      serviceConfig = {
        ExecStart = "${pkgs.myService}/bin/my-service --port ${toString config.myService.port}";
        DynamicUser = true;
        NoNewPrivileges = true;
      };
    };
  };
}
```

**Rules:**
- Always use `lib.mkIf config.<module>.enable` to gate configuration. Modules that unconditionally write to `config` are impossible to disable.
- Declare options before using them. Never access `config.<module>.*` in the same module's `options` block.
- Keep modules focused on a single service or concern. Compose them rather than building monolithic modules.

### Option Declaration

Use the richest option type that matches the data:

```nix
options.wireguard = {
  # Boolean — use mkEnableOption for on/off options
  enable = lib.mkEnableOption "Wireguard VPN";

  # String — use mkOption with type
  interface = lib.mkOption {
    type = lib.types.str;
    default = "wg0";
    example = "wg1";
    description = lib.mdDoc "Name of the Wireguard network interface.";
  };

  # Port — validated integer 1–65535
  listenPort = lib.mkOption {
    type = lib.types.port;
    default = 51820;
    description = lib.mdDoc "UDP port Wireguard listens on.";
  };

  # Attribute set of sub-options — for structured config
  peers = lib.mkOption {
    type = lib.types.attrsOf (lib.types.submodule {
      options = {
        publicKey = lib.mkOption { type = lib.types.str; };
        allowedIPs = lib.mkOption { type = lib.types.listOf lib.types.str; };
        endpoint = lib.mkOption {
          type = lib.types.nullOr lib.types.str;
          default = null;
        };
      };
    });
    default = {};
    description = lib.mdDoc "Wireguard peers indexed by name.";
  };
};
```

**Rules:**
- Always include `description` using `lib.mdDoc "..."`. Agents and operators rely on these to understand options.
- Always include a `default` unless the option is mandatory. If mandatory, document why in the description.
- Include an `example` for non-obvious values.
- Use `lib.types.nullOr` for optional values rather than empty strings or sentinel values.

### Conditional Configuration

Use `lib.mkIf`, `lib.mkMerge`, and `lib.mkAfter` for conditional and composable configuration:

```nix
config = lib.mkMerge [
  # Always applied when the module is enabled
  (lib.mkIf config.myModule.enable {
    networking.firewall.allowedUDPPorts = [ config.myModule.listenPort ];
  })

  # Only applied when SSH is also enabled
  (lib.mkIf (config.myModule.enable && config.services.openssh.enable) {
    # Restrict SSH to VPN interface when module is active
    services.openssh.listenAddresses = [
      { addr = config.myModule.vpnAddress; port = 22; }
    ];
  })
];
```

### Module Assertions

Validate invariants at evaluation time, not at service runtime:

```nix
config = lib.mkIf config.myModule.enable {
  assertions = [
    {
      assertion = lib.hasPrefix "https://" config.myModule.vaultAddress
        || config.myModule.vaultAddress == "http://127.0.0.1:8200";
      message = "myModule.vaultAddress must use HTTPS in production.";
    }
    {
      assertion = config.myModule.peers != {};
      message = "myModule.peers must not be empty — at least one peer is required.";
    }
  ];
};
```

Assertions fail at `nix flake check` or `nixos-rebuild` evaluation, before any deployment occurs.

---

## Flake Structure Patterns

### Standard Flake Layout

```nix
# flake.nix
{
  description = "NixOS configuration for [hostname]";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    agenix = {
      url = "github:ryantm/agenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    home-manager = {
      url = "github:nix-community/home-manager/release-24.11";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, agenix, home-manager, ... }: {
    # NixOS configurations indexed by hostname
    nixosConfigurations = {
      my-laptop = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [
          ./hosts/my-laptop/configuration.nix
          agenix.nixosModules.default
          home-manager.nixosModules.home-manager
        ];
      };
    };

    # Flake checks — run with `nix flake check`
    checks.x86_64-linux = {
      wireguard-test = import ./tests/nixos/wireguard.nix {
        pkgs = nixpkgs.legacyPackages.x86_64-linux;
      };
    };

    # Development shell
    devShells.x86_64-linux.default = nixpkgs.legacyPackages.x86_64-linux.mkShell {
      packages = with nixpkgs.legacyPackages.x86_64-linux; [
        nixos-rebuild agenix wireguard-tools vault
      ];
    };
  };
}
```

### Device Profile Pattern

Share common configuration across device types using profile modules:

```
nixos-config/
├── flake.nix
├── hosts/
│   ├── laptop-dev/
│   │   ├── configuration.nix    # Host-specific: hardware, hostname, users
│   │   └── hardware-configuration.nix
│   └── home-server/
│       ├── configuration.nix
│       └── hardware-configuration.nix
├── profiles/
│   ├── base.nix                  # All hosts: nix settings, locale, timezone
│   ├── workstation.nix           # Laptops: Hyprland, audio, Bluetooth
│   ├── server.nix                # Servers: headless, sshd, firewall
│   └── router.nix                # Routers: NAT, forwarding, DHCP
├── modules/
│   ├── wireguard/
│   ├── vault/
│   └── hyprland/
└── secrets/
```

Each host imports the relevant profile and its specific hardware configuration:

```nix
# hosts/laptop-dev/configuration.nix
{ ... }:
{
  imports = [
    ./hardware-configuration.nix
    ../../profiles/base.nix
    ../../profiles/workstation.nix
    ../../modules/wireguard/client.nix
    ../../modules/vault/agent.nix
  ];

  networking.hostName = "laptop-dev";
  # Host-specific overrides...
}
```

### Shared Module Library

Place reusable modules in `modules/` and import them by path. Never use flake inputs for local modules — relative paths are sufficient and clearer:

```nix
# modules/wireguard/client.nix — usable by any host that needs a VPN client
{ config, lib, pkgs, ... }:
{
  options.custom.wireguard.client = {
    enable = lib.mkEnableOption "Wireguard client";
    # ...
  };
}
```

**Rules:**
- One module per service or concern. A `networking.nix` that configures Wireguard, firewall, and DNS is three concerns — split it.
- Profiles compose modules for a device type. Host configurations add host-specific overrides.
- Never put hardware configuration in profiles — it belongs in the host directory.

---

## Systemd Service Patterns

### Service with Secret Injection

Inject Vault secrets before the service starts via `preStart`:

```nix
systemd.services."my-service" = {
  description = "My Service";
  after = [ "network.target" "vault-agent.service" ];
  requires = [ "vault-agent.service" ];
  wantedBy = [ "multi-user.target" ];

  preStart = ''
    # Fetch secret from Vault and write to /run/secrets/
    install -d -m 700 /run/secrets/my-service
    ${pkgs.vault}/bin/vault kv get \
      -field=value secret/my-service/api-key \
      > /run/secrets/my-service/api-key
    chmod 400 /run/secrets/my-service/api-key
  '';

  serviceConfig = {
    ExecStart = "${pkgs.myService}/bin/my-service";
    User = "my-service";
    Group = "my-service";
    RuntimeDirectory = "my-service";
    EnvironmentFile = "/run/secrets/my-service/api-key";
  };
};
```

### Service Hardening Template

Apply these hardening options to every service module. Remove only what the service specifically requires:

```nix
serviceConfig = {
  # Filesystem isolation
  ProtectSystem = "strict";
  ProtectHome = true;
  PrivateTmp = true;
  PrivateDevices = true;
  ReadWritePaths = [ "/run/my-service" ];  # Only writable paths the service needs

  # Privilege restrictions
  NoNewPrivileges = true;
  CapabilityBoundingSet = "";   # No capabilities unless explicitly needed
  AmbientCapabilities = "";

  # Syscall filtering
  SystemCallFilter = [ "@system-service" "~@privileged" ];
  SystemCallErrorNumber = "EPERM";

  # User isolation
  DynamicUser = true;           # Use unless persistent state requires a fixed UID
  # If persistent state: User = "my-service"; Group = "my-service";

  # Restrict address families
  RestrictAddressFamilies = [ "AF_INET" "AF_INET6" "AF_UNIX" ];
};
```

### Service Ordering and Dependencies

Use `after`, `requires`, `wants`, and `before` correctly:

| Directive | Meaning |
|-----------|---------|
| `after = [ "X" ]` | Start after X if X is running, but do not require X |
| `requires = [ "X" ]` | Start after X and fail if X fails |
| `wants = [ "X" ]` | Start X alongside this service (soft dependency) |
| `before = [ "X" ]` | Start before X |
| `wantedBy = [ "multi-user.target" ]` | Start during normal boot |

**Rules:**
- Use `requires` for hard dependencies (Vault agent must be running before secret injection).
- Use `after` + `network.target` for services that need the network but should not fail if networking is briefly unavailable.
- Never add `wantedBy` to a module unconditionally — gate it behind `lib.mkIf config.<module>.enable`.

---

## Secret Injection Architecture

The two-tier pattern:

```
┌─────────────────────────────────────────────────────────────┐
│  Tier 1: agenix (bootstrap only)                            │
│  secrets/vault-token.age → /run/agenix/vault-token          │
│  Encrypted with admin SSH keys. One secret only.            │
└───────────────────────────┬─────────────────────────────────┘
                            │ reads
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  Vault Agent (systemd service)                              │
│  Authenticates to Vault using the bootstrap token           │
│  Manages lease renewal and token rotation                   │
└───────────────────────────┬─────────────────────────────────┘
                            │ injects at startup
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  Tier 2: Vault (all other secrets)                          │
│  /run/secrets/wireguard-private-key  (mode 400, root)       │
│  /run/secrets/wireguard-psk          (mode 400, root)       │
│  /run/secrets/my-service/api-key     (mode 400, service)    │
│  All written to /run/secrets/ (tmpfs, not persistent)       │
└─────────────────────────────────────────────────────────────┘
```

**Rules:**
- Secrets in `/run/secrets/` live in tmpfs and are lost on reboot. The Vault agent re-injects on next boot — this is correct behaviour.
- Never write secrets to `/etc/`, `/home/`, or any persistent path. Only `/run/secrets/`.
- Each service accesses only the secrets it needs — separate subdirectories per service.

---

## Wireguard Architecture Patterns

### Hub-and-Spoke Pattern

One server peer (hub) routes traffic between client peers (spokes). All traffic between spokes passes through the hub.

```
Client A (10.100.0.2) ──→── Hub (10.100.0.1) ──→── Client B (10.100.0.3)
```

Use when: home lab with a VPS as hub; clients include mobile devices and laptops.

```nix
# Hub configuration
networking.wireguard.interfaces.wg0 = {
  ips = [ "10.100.0.1/24" ];
  listenPort = 51820;
  peers = [
    { publicKey = clientAKey; allowedIPs = [ "10.100.0.2/32" ]; }
    { publicKey = clientBKey; allowedIPs = [ "10.100.0.3/32" ]; }
  ];
};
boot.kernel.sysctl."net.ipv4.ip_forward" = 1;
```

### Full Tunnel Pattern

All client traffic routes through the VPN server (including internet traffic). Used for privacy on untrusted networks.

```nix
# Client configuration — full tunnel
networking.wireguard.interfaces.wg0 = {
  ips = [ "10.100.0.2/32" ];
  peers = [{
    publicKey = serverKey;
    allowedIPs = [ "0.0.0.0/0" "::/0" ];  # Route all traffic
    endpoint = "vpn.example.com:51820";
    persistentKeepalive = 25;
  }];
};
```

### Site-to-Site Pattern

Two networks connected via a Wireguard tunnel. Traffic between subnets routes over the VPN.

```nix
# Site A gateway
networking.wireguard.interfaces.wg0 = {
  ips = [ "10.200.0.1/30" ];
  listenPort = 51820;
  peers = [{
    publicKey = siteBGatewayKey;
    allowedIPs = [ "10.200.0.2/32" "192.168.2.0/24" ];  # VPN IP + Site B subnet
    endpoint = "site-b.example.com:51820";
  }];
};
# Route Site B subnet over VPN
networking.interfaces.wg0.ipv4.routes = [{
  address = "192.168.2.0";
  prefixLength = 24;
}];
```

---

## Overlay Patterns

Use overlays to override or extend packages from nixpkgs. Apply overlays in the flake, not in individual modules:

```nix
# flake.nix
outputs = { self, nixpkgs, ... }: let
  overlays = [
    # Override a package version
    (final: prev: {
      vault = prev.vault.overrideAttrs (old: {
        version = "1.18.0";
        src = prev.fetchurl { /* ... */ };
      });
    })
    # Add a local package
    (final: prev: {
      syntek-infra-tool = final.callPackage ./plugins { };
    })
  ];
in {
  nixosConfigurations.my-host = nixpkgs.lib.nixosSystem {
    system = "x86_64-linux";
    modules = [
      { nixpkgs.overlays = overlays; }
      ./hosts/my-host/configuration.nix
    ];
  };
};
```

**Rules:**
- Define overlays in `flake.nix`, not in per-module files. Overlays in modules are hard to trace.
- Apply overlays via `nixpkgs.overlays` in a module or directly in `nixosSystem`. Never use `import nixpkgs { overlays = [...]; }` inside a module.
- Prefer `overrideAttrs` for minor changes. Use `callPackage` for wholly new packages.

---

## Cross-System Builds

For configurations targeting multiple architectures (x86_64, aarch64):

```nix
outputs = { self, nixpkgs, ... }: {
  nixosConfigurations = {
    # x86_64 laptop
    dev-laptop = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [ ./hosts/dev-laptop/configuration.nix ];
    };

    # aarch64 Raspberry Pi
    home-server = nixpkgs.lib.nixosSystem {
      system = "aarch64-linux";
      modules = [ ./hosts/home-server/configuration.nix ];
    };
  };

  # Cross-compile aarch64 from x86_64 using binfmt emulation
  # Enable in the build host: boot.binfmt.emulatedSystems = [ "aarch64-linux" ];
};
```

---

## Hyprland Configuration Patterns

Structure Hyprland configuration as NixOS modules, not raw config files:

```nix
# modules/hyprland/default.nix
{ config, lib, pkgs, ... }:
{
  options.custom.hyprland = {
    enable = lib.mkEnableOption "Hyprland compositor";
    monitors = lib.mkOption {
      type = lib.types.listOf (lib.types.submodule {
        options = {
          name = lib.mkOption { type = lib.types.str; };
          resolution = lib.mkOption { type = lib.types.str; default = "1920x1080@60"; };
          position = lib.mkOption { type = lib.types.str; default = "0x0"; };
          scale = lib.mkOption { type = lib.types.float; default = 1.0; };
        };
      });
      default = [];
    };
  };

  config = lib.mkIf config.custom.hyprland.enable {
    programs.hyprland.enable = true;

    # Generate monitor config from structured options
    home-manager.users.${config.custom.primaryUser} = {
      wayland.windowManager.hyprland.settings.monitor =
        map (m: "${m.name},${m.resolution},${m.position},${toString m.scale}")
          config.custom.hyprland.monitors;
    };
  };
}
```

**Rules:**
- Model Hyprland configuration as structured NixOS options, not raw string interpolation.
- Separate monitor configuration, keybindings, and window rules into distinct option groups.
- Test configuration in a nested Hyprland session before applying to the main compositor.

---

## Architecture Checklist

Before merging any new module or flake change:

- [ ] Module uses `lib.mkIf config.<module>.enable` to gate all configuration
- [ ] All options have `description`, `type`, and `default`
- [ ] Module assertions validate invariants at evaluation time
- [ ] Systemd services use hardening options (`ProtectSystem`, `NoNewPrivileges`, `DynamicUser`)
- [ ] Secrets are injected from Vault via `preStart`, not hardcoded
- [ ] Firewall rules open only required ports with comments explaining why
- [ ] Device profile pattern used: profile → module composition, host → profile + hardware
- [ ] Overlays defined in `flake.nix`, not in modules
- [ ] `nix flake check` passes without errors or warnings
- [ ] Configuration tested in a VM before deploying to hardware
