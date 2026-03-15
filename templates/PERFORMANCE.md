# Performance

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Infrastructure Team
**Language:** British English (en_GB)
**Timezone:** Europe/London
**Plugin Scope:** syntek-infra (NixOS, Hyprland, Wireguard, Vault, Vaultwarden)

---

## Table of Contents

- [Overview](#overview)
- [The Rules](#the-rules)
- [Nix Build Performance](#nix-build-performance)
  - [Binary Caches and Substituters](#binary-caches-and-substituters)
  - [Distributed Builds](#distributed-builds)
  - [Build Parallelism](#build-parallelism)
- [Nix Evaluation Performance](#nix-evaluation-performance)
  - [Avoiding IFD](#avoiding-ifd)
  - [Lazy Evaluation](#lazy-evaluation)
  - [Module Evaluation Overhead](#module-evaluation-overhead)
- [Nix Store Management](#nix-store-management)
  - [Garbage Collection](#garbage-collection)
  - [Store Optimisation](#store-optimisation)
- [Systemd Service Tuning](#systemd-service-tuning)
  - [Startup Time](#startup-time)
  - [Service Resource Limits](#service-resource-limits)
- [Wireguard Performance](#wireguard-performance)
- [Vault Performance](#vault-performance)
- [Hyprland Performance](#hyprland-performance)
- [Monitoring and Measurement](#monitoring-and-measurement)
- [Performance Checklist](#performance-checklist)

---

## Overview

Performance work in NixOS infrastructure has two distinct dimensions: **build-time performance** (how fast the system can be rebuilt and deployed) and **runtime performance** (how efficiently services run on the deployed system).

Apply the same philosophy as the coding principles: measure first, then optimise. `nix build --dry-run` and `systemd-analyze` are your starting points before making changes.

---

## The Rules

1. **Measure before optimising.** (Pike Rule 1 and Rule 2.) Use `nix build --dry-run`, `systemd-analyze blame`, and `journalctl` before assuming where time is spent.
2. **Binary caches eliminate most build time.** Before adding a distributed build farm, check whether the binary cache covers your packages.
3. **Evaluation errors cost nothing to fix early.** A module that fails `nix flake check` in 5 seconds is better than one that fails after a 20-minute build.
4. **Service startup order is the most common runtime bottleneck.** Use `systemd-analyze critical-chain` to identify it.
5. **The fastest code is code that does not run.** Disable unused services — each one adds startup time and consumes resources.

---

## Nix Build Performance

### Binary Caches and Substituters

Configure substituters to download pre-built binaries rather than building from source. Most packages in nixpkgs have binary cache entries on cache.nixos.org.

```nix
# In configuration.nix or nix.settings
nix.settings = {
  # Official cache (enabled by default)
  substituters = [
    "https://cache.nixos.org"
    "https://nix-community.cachix.org"   # Community cache for home-manager, agenix etc.
  ];

  trusted-public-keys = [
    "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
    "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCUSeBs="
  ];

  # Speed up cache lookups — skip checking substituters for paths we know aren't cached
  narinfo-cache-positive-ttl = 86400;    # 24 hours
  narinfo-cache-negative-ttl = 3600;     # 1 hour
};
```

For private packages, set up a self-hosted binary cache (Cachix or `nix-serve`):

```nix
# nix-serve on a trusted server
services.nix-serve = {
  enable = true;
  port = 5000;
  secretKeyFile = "/run/secrets/nix-serve-key";
};
```

**Rules:**
- Always add `nix-community.cachix.org` when using home-manager, agenix, or other community flakes — it dramatically reduces build times.
- Add your project's own cache for any packages not in nixpkgs or community caches.
- Verify cache hits with `nix build --dry-run` — lines marked `will be built` indicate cache misses.

### Distributed Builds

When cache misses are unavoidable, distribute builds to faster machines:

```nix
nix.distributedBuilds = true;
nix.buildMachines = [
  {
    hostName = "build-server.vpn";
    systems = [ "x86_64-linux" "aarch64-linux" ];
    maxJobs = 8;
    speedFactor = 4;
    sshKey = "/run/secrets/nix-build-key";
    sshUser = "nix-build";
    supportedFeatures = [ "nixos-test" "big-parallel" "kvm" ];
  }
];
```

The build host must have the `nix-build` user with the builder's SSH public key in its `authorized_keys`.

**Rules:**
- Distributed builds require trust. The `nix-build` SSH key must be in Vault and rotated on schedule.
- Only use distributed builds for packages that regularly miss the cache. Most builds should hit the cache.
- Enable `kvm` feature on build machines that will run `nixosTest` VM tests.

### Build Parallelism

```nix
nix.settings = {
  max-jobs = "auto";              # Use all CPU cores for parallel builds
  cores = 0;                      # Per-job cores: 0 = all available
  build-cores = 0;
};
```

For low-RAM machines, limit parallelism to avoid OOM during large builds:

```nix
nix.settings = {
  max-jobs = 2;        # Limit to 2 parallel jobs on 4 GB RAM systems
  cores = 2;
};
```

---

## Nix Evaluation Performance

### Avoiding IFD

Import-From-Derivation (IFD) occurs when a Nix derivation is imported as if it were a Nix expression. This forces evaluation to wait for a build — breaking lazy evaluation and dramatically slowing flake checks.

```nix
# BAD — IFD: evaluation blocks waiting for the build
let data = builtins.fromJSON (builtins.readFile "${pkgs.generateConfig}/config.json");

# GOOD — generate config at build time, evaluate from source
let data = builtins.fromJSON (builtins.readFile ./config.json);
```

**Rule:** If `nix flake check --no-build` fails due to an unbuilt derivation being imported, you have IFD. Restructure to use pre-existing source files or pass data as flake inputs.

### Lazy Evaluation

Nix uses lazy evaluation — expressions are only evaluated when their value is needed. Write modules to take advantage of this:

```nix
# GOOD — only evaluated when the module is enabled
config = lib.mkIf config.myModule.enable {
  # This large attribute set is never evaluated if enable = false
  systemd.services = generateManyServices config.myModule;
};

# BAD — always evaluated regardless of enable flag
systemd.services = lib.optionalAttrs config.myModule.enable (generateManyServices config.myModule);
# optionalAttrs still evaluates generateManyServices before discarding it
```

### Module Evaluation Overhead

Keep module option types simple. Complex recursive types (deep `attrsOf (submodule ...)`) increase evaluation time quadratically with the number of entries.

```nix
# If you have hundreds of peers, a flat list is faster to evaluate than a deeply nested attrsOf
options.wireguard.peers = lib.mkOption {
  type = lib.types.listOf (lib.types.submodule { ... });  # Faster for large lists
  # vs. lib.types.attrsOf (lib.types.submodule { ... })  — slower for large attribute sets
};
```

---

## Nix Store Management

### Garbage Collection

Configure automatic garbage collection to prevent the store from growing unboundedly:

```nix
nix.gc = {
  automatic = true;
  dates = "weekly";
  options = "--delete-older-than 30d";
};

# Preserve the last N system generations before GC
nix.settings.keep-outputs = true;
nix.settings.keep-derivations = true;
```

Check store size before and after GC:

```bash
du -sh /nix/store
nix-collect-garbage --delete-older-than 30d
du -sh /nix/store
```

### Store Optimisation

Deduplicate identical files in the Nix store with hard links:

```nix
nix.settings.auto-optimise-store = true;
```

Or run manually:

```bash
nix store optimise
```

**Rules:**
- Enable `auto-optimise-store` on servers with large stores (build servers, development machines).
- Do not enable on machines with very slow storage — optimisation scans the entire store.
- Keep at least 2 system generations before GC to allow rollback.

---

## Systemd Service Tuning

### Startup Time

Measure boot performance:

```bash
# Overall boot time
systemd-analyze

# Per-service breakdown
systemd-analyze blame

# Critical path (longest chain of sequential dependencies)
systemd-analyze critical-chain

# Plot boot timeline (outputs SVG)
systemd-analyze plot > boot.svg
```

Common causes of slow startup:
- Services `after = [ "network-online.target" ]` when they only need `network.target` — `network-online.target` waits for DHCP/static IP assignment, which can take seconds.
- Sequential service chains where parallel startup is possible.
- Services that do heavy initialisation in `ExecStart` that could be done in `ExecStartPre` in parallel.

```nix
# Only use network-online.target if the service actually needs a fully configured network
# (e.g., it connects to an external host at startup)
systemd.services.my-service = {
  after = [ "network.target" ];         # Most services: just needs network stack up
  # after = [ "network-online.target" ]; # Only if it needs a fully routable network
};
```

### Service Resource Limits

Set resource limits to prevent a single service from starving the system:

```nix
systemd.services.my-service = {
  serviceConfig = {
    # Memory limit — service is OOM-killed if it exceeds this
    MemoryMax = "512M";
    MemoryHigh = "400M";      # Soft limit — triggers memory pressure handling

    # CPU throttling
    CPUQuota = "50%";         # Maximum 50% of one CPU core

    # IO weight (default 100, lower = less IO priority)
    IOWeight = 50;

    # File descriptor limit
    LimitNOFILE = 65536;
  };
};
```

---

## Wireguard Performance

Wireguard runs in the kernel and has minimal overhead. Performance considerations:

- **MTU:** Set Wireguard interface MTU to 1420 (for IPv4) or 1400 (for IPv6 over IPv4) to avoid fragmentation. The default in NixOS is correct for most configurations.

```nix
networking.wireguard.interfaces.wg0 = {
  mtu = 1420;
  # ...
};
```

- **Persistent keepalive:** Only enable on clients behind NAT. `persistentKeepalive = 25` sends a packet every 25 seconds to keep the NAT mapping alive.

- **Multiple peers:** Wireguard scales linearly with the number of peers. Hundreds of peers on a modern CPU have negligible CPU overhead.

- **Monitoring throughput:**

```bash
# Bytes transferred per peer
wg show wg0 transfer

# Last handshake time (stale = > 3 minutes indicates connectivity issue)
wg show wg0 latest-handshakes
```

---

## Vault Performance

For small-to-medium infrastructure, Vault's default configuration is sufficient. Tune when:

- Vault agent startup is slow (> 5 seconds): increase `renew_token` frequency or check network latency to the Vault server.
- Secret injection delays service startup: use `preStart` ordering correctly (see ARCHITECTURE-PATTERNS.md — Service with Secret Injection).

```nix
# Vault agent — tune cache and renewal settings
services.vault-agent = {
  settings = {
    cache = {
      use_auto_auth_token = true;
    };
    vault = {
      address = "https://vault.example.com";
      retry = {
        num_retries = 5;
      };
    };
  };
};
```

---

## Hyprland Performance

- **VSync:** Enable `vrr = 1` (Variable Refresh Rate) if your monitor supports it. This reduces latency and tearing.

```nix
wayland.windowManager.hyprland.settings.misc = {
  vrr = 1;
  vfr = true;   # Variable frame rate — reduces GPU usage when idle
};
```

- **GPU driver:** Ensure the correct kernel module and Mesa driver are loaded. Use `nvidia` packages for NVIDIA, `amdgpu` for AMD, or `i915` for Intel. Performance on the wrong driver is dramatically worse.

```nix
# AMD GPU example
hardware.amdgpu.enable = true;
hardware.opengl.enable = true;
hardware.opengl.driSupport = true;
hardware.opengl.driSupport32Bit = true;
```

- **Animations:** Reduce animation duration for lower-latency desktop experience:

```nix
wayland.windowManager.hyprland.settings.animations = {
  enabled = true;
  bezier = [ "easeOut, 0.05, 0.9, 0.1, 1.05" ];
  animation = [
    "windows, 1, 3, easeOut"      # Shorter duration (3 = 300ms)
    "workspaces, 1, 3, easeOut"
  ];
};
```

---

## Monitoring and Measurement

### Key Metrics

| Metric | Tool | Target |
|--------|------|--------|
| Boot time | `systemd-analyze` | < 30s for workstations, < 60s for servers |
| Service startup | `systemd-analyze blame` | No individual service > 10s |
| Nix build time (cache hit) | `nix build --dry-run` | 0 builds (full cache hit) |
| Store size | `du -sh /nix/store` | Managed by GC schedule |
| Wireguard handshake age | `wg show latest-handshakes` | < 3 minutes for active peers |
| Vault agent response | `vault kv get` timing | < 500ms |

### Useful Commands

```bash
# Boot performance
systemd-analyze
systemd-analyze blame | head -20
systemd-analyze critical-chain

# Service resource usage
systemctl status my-service
systemd-cgtop                          # Real-time resource usage by service

# Nix build cache efficiency
nix build .#nixosConfigurations.my-host.config.system.build.toplevel --dry-run 2>&1 | grep "will be built"

# Store usage
nix path-info --size --human-readable /run/current-system
du -sh /nix/store

# Network interface stats
ip -s link show wg0                    # Wireguard bytes in/out
wg show wg0                            # Peer status
```

---

## Performance Checklist

Before deploying a configuration change:

- [ ] `systemd-analyze blame` reviewed — no unexpected services dominate startup
- [ ] Binary cache substituters configured (`cache.nixos.org`, community caches)
- [ ] `nix build --dry-run` shows no unexpected cache misses for common packages
- [ ] Nix GC scheduled (`nix.gc.automatic = true`)
- [ ] Store optimisation enabled (`auto-optimise-store = true`) if appropriate
- [ ] Services use `network.target` not `network-online.target` unless required
- [ ] Resource limits set on services that handle untrusted input or do heavy processing
- [ ] Wireguard MTU set to 1420 (IPv4) or 1400 (IPv6)
- [ ] `persistentKeepalive` only enabled on NAT-traversal clients, not servers
- [ ] Hyprland VFR enabled (`vfr = true`) to reduce GPU idle usage
- [ ] Unused services disabled — each enabled service adds startup time
