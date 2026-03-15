# Data Structures

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Infrastructure Team
**Language:** British English (en_GB)
**Timezone:** Europe/London
**Plugin Scope:** syntek-infra (NixOS, Rust, Hyprland, Wireguard, Vault)

---

## Table of Contents

- [Overview](#overview)
- [The Principle](#the-principle)
- [Nix Type System](#nix-type-system)
  - [Primitive Types](#primitive-types)
  - [Attribute Sets](#attribute-sets)
  - [Lists](#lists)
  - [Functions and Lambdas](#functions-and-lambdas)
  - [Derivations](#derivations)
- [NixOS Option Types](#nixos-option-types)
  - [Scalar Types](#scalar-types)
  - [Collection Types](#collection-types)
  - [Submodule Types](#submodule-types)
  - [Custom Types](#custom-types)
- [Module Option Design](#module-option-design)
  - [Good Option Structure](#good-option-structure)
  - [Composable Configurations](#composable-configurations)
- [Rust Types for Infrastructure Tools](#rust-types-for-infrastructure-tools)
  - [Newtypes for Domain Values](#newtypes-for-domain-values)
  - [Enumerations for State](#enumerations-for-state)
  - [Error Types](#error-types)
  - [Configuration Structs](#configuration-structs)
- [Configuration Data Modelling](#configuration-data-modelling)
  - [Device Configuration](#device-configuration)
  - [Service Configuration](#service-configuration)
  - [Wireguard Peer Configuration](#wireguard-peer-configuration)
- [Anti-Patterns](#anti-patterns)
  - [Stringly Typed Options](#stringly-typed-options)
  - [God Attribute Set](#god-attribute-set)
  - [Boolean Blindness](#boolean-blindness)
  - [Nested String Interpolation](#nested-string-interpolation)
  - [Mutable State in Nix](#mutable-state-in-nix)
- [Rules and Principles](#rules-and-principles)

---

## Overview

Data structure decisions in NixOS infrastructure have two layers: **Nix** (configuration language) and **Rust** (tooling language). Get the data structures right in both and the configuration logic and tool behaviour become obvious. Get them wrong and every module becomes a workaround.

This guide covers the Nix type system, NixOS option type design, and Rust type patterns for the syntek-infra-tool CLI.

---

## The Principle

Rob Pike and Linus Torvalds express the same idea:

> "If you have chosen the right data structures and organised things well, the algorithms will almost always be self-evident." — Rob Pike

> "Show me your tables, and I won't usually need your flowcharts; they'll be obvious." — Linus Torvalds (via Fred Brooks)

In NixOS: design the option schema before writing module logic. In Rust: design the types before writing functions. The logic follows from the structure.

---

## Nix Type System

Nix is a purely functional, lazily evaluated language. Understanding its types is fundamental to writing correct NixOS modules.

### Primitive Types

| Type | Example | Notes |
|------|---------|-------|
| `null` | `null` | Absence of a value. Use `lib.types.nullOr` in options. |
| `bool` | `true`, `false` | Boolean. `lib.mkEnableOption` produces a bool option. |
| `int` | `8080`, `-1` | Arbitrary precision integer. |
| `float` | `1.5`, `0.0` | Floating-point. Rare in configuration; used for scaling factors. |
| `string` | `"hello"` | UTF-8 string. May include interpolation `${...}`. |
| `path` | `./modules/foo.nix` | Filesystem path. Different from string — paths are copied into the Nix store. |

```nix
# String vs path — critical distinction
imports = [ ./hardware-configuration.nix ];  # path: correct
imports = [ "/etc/nixos/hardware-configuration.nix" ];  # string: wrong — not a Nix path
```

### Attribute Sets

Attribute sets (attrsets) are the primary data structure in Nix — equivalent to dictionaries or records in other languages.

```nix
# Literal attrset
{ name = "wg0"; port = 51820; }

# Nested attrset
{
  networking.wireguard.interfaces.wg0 = {
    ips = [ "10.100.0.1/24" ];
    listenPort = 51820;
  };
}

# Recursive attrset — can reference its own attributes
rec {
  base = "10.100.0";
  server = "${base}.1/24";
  client = "${base}.2/24";
}
```

**Key attrset operations:**

```nix
# Merge two attrsets (right-hand overrides left)
{ a = 1; b = 2; } // { b = 3; c = 4; }   # => { a = 1; b = 3; c = 4; }

# Access an attribute
config.networking.wireguard.interfaces

# Check if attribute exists
lib.hasAttr "wg0" config.networking.wireguard.interfaces

# Map over an attrset
lib.mapAttrs (name: value: value // { inherit name; }) peers
```

### Lists

Lists are ordered, typed sequences. Unlike most languages, Nix lists are not efficient for large data sets — prefer attrsets when keyed lookup is needed.

```nix
# Literal list
[ "10.100.0.2/32" "10.100.0.3/32" ]

# Concatenate lists
[ "a" "b" ] ++ [ "c" "d" ]   # => [ "a" "b" "c" "d" ]

# Map over a list
map (ip: "${ip}/32") [ "10.100.0.2" "10.100.0.3" ]

# Filter a list
lib.filter (port: port != 22) openPorts

# Optional element
lib.optional config.services.openssh.enable 22
# => [ 22 ] if ssh is enabled, [] otherwise
```

### Functions and Lambdas

Functions are first-class values in Nix. Every NixOS module is a function from its inputs to a configuration attrset.

```nix
# Lambda: argument: body
(x: x + 1)

# Named attribute set argument (most common in modules)
{ config, lib, pkgs, ... }:
{
  # module body
}

# Default arguments
({ name ? "default", port ? 8080 }: "${name}:${toString port}")

# Let binding
let
  vaultAddr = "https://vault.example.com";
  mkSecretPath = service: "/run/secrets/${service}";
in {
  # use vaultAddr and mkSecretPath here
}
```

### Derivations

Derivations are build recipes — they describe how to build a package. You interact with them via `pkgs.*` or `callPackage`. Do not write derivations in module configuration files.

```nix
# Use derivations from pkgs — do not define them in modules
serviceConfig.ExecStart = "${pkgs.vault}/bin/vault agent";

# For local packages, define derivations in a dedicated package.nix and call from flake
packages.x86_64-linux.syntek-infra-tool = pkgs.callPackage ./plugins/syntek-infra-tool { };
```

---

## NixOS Option Types

NixOS option types are defined in `lib.types`. Use the most specific type that fits — this enables better error messages and documentation.

### Scalar Types

| Type | Usage | Example |
|------|-------|---------|
| `lib.types.bool` | Boolean flags | `enable = lib.mkEnableOption "service"` |
| `lib.types.str` | Free-form strings | Interface names, hostnames, addresses |
| `lib.types.nonEmptyStr` | Strings that must not be empty | Required names |
| `lib.types.int` | Integers | Numeric configuration values |
| `lib.types.ints.unsigned` | Non-negative integers | Timeouts, retries |
| `lib.types.port` | Port number 1–65535 | Service listen ports |
| `lib.types.path` | Filesystem path | Key file paths |
| `lib.types.float` | Floating-point | Hyprland scale factors |
| `lib.types.enum [ "a" "b" ]` | One of a fixed set | WireGuard interface mode |

### Collection Types

```nix
# List of strings
allowedIPs = lib.mkOption {
  type = lib.types.listOf lib.types.str;
  default = [];
};

# Nullable value — present or absent
endpoint = lib.mkOption {
  type = lib.types.nullOr lib.types.str;
  default = null;
};

# Attribute set of strings
environment = lib.mkOption {
  type = lib.types.attrsOf lib.types.str;
  default = {};
};

# One of a fixed set of string values
logLevel = lib.mkOption {
  type = lib.types.enum [ "debug" "info" "warning" "error" ];
  default = "info";
};
```

### Submodule Types

Use `lib.types.submodule` for structured nested options:

```nix
options.wireguard.peers = lib.mkOption {
  type = lib.types.attrsOf (lib.types.submodule {
    options = {
      publicKey = lib.mkOption {
        type = lib.types.str;
        description = lib.mdDoc "The peer's Wireguard public key (base64-encoded).";
      };
      allowedIPs = lib.mkOption {
        type = lib.types.listOf lib.types.str;
        description = lib.mdDoc "IP address ranges routed to this peer.";
        example = [ "10.100.0.2/32" ];
      };
      endpoint = lib.mkOption {
        type = lib.types.nullOr lib.types.str;
        default = null;
        description = lib.mdDoc "Optional IP:port of the peer. Required for initiating connections.";
        example = "vpn.example.com:51820";
      };
      persistentKeepalive = lib.mkOption {
        type = lib.types.nullOr lib.types.ints.unsigned;
        default = null;
        description = lib.mdDoc "Keepalive interval in seconds. Set to 25 for clients behind NAT.";
      };
    };
  });
  default = {};
  description = lib.mdDoc "Wireguard peers, indexed by peer name.";
};
```

### Custom Types

For validation beyond what built-in types provide, use `lib.types.strMatching` or build a custom type:

```nix
# Only accept valid CIDR notation
cidrType = lib.types.strMatching "[0-9./:]+" // {
  description = "CIDR notation (e.g. 10.100.0.0/24)";
};

# Only accept base64-encoded strings (for Wireguard keys)
base64Type = lib.types.strMatching "[A-Za-z0-9+/]+=*" // {
  description = "Base64-encoded string";
};
```

---

## Module Option Design

### Good Option Structure

Options should be self-documenting and validate their own constraints:

```nix
options.vault = {
  enable = lib.mkEnableOption "Hashicorp Vault agent";

  address = lib.mkOption {
    type = lib.types.str;
    example = "https://vault.example.com";
    description = lib.mdDoc ''
      Address of the Vault server. Must use HTTPS in production.
      Use `http://127.0.0.1:8200` for local development only.
    '';
  };

  roleIdFile = lib.mkOption {
    type = lib.types.path;
    description = lib.mdDoc "Path to the Vault AppRole role ID file (injected by agenix).";
    example = "/run/agenix/vault-role-id";
  };

  secretIdFile = lib.mkOption {
    type = lib.types.path;
    description = lib.mdDoc "Path to the Vault AppRole secret ID file (injected by agenix).";
    example = "/run/agenix/vault-secret-id";
  };

  renewInterval = lib.mkOption {
    type = lib.types.ints.unsigned;
    default = 300;
    description = lib.mdDoc "How often (in seconds) the Vault agent attempts to renew the token.";
  };
};
```

### Composable Configurations

Design options to compose cleanly with `lib.mkMerge` and `lib.mkIf`:

```nix
# BAD — monolithic option that mixes concerns
options.myModule.config = lib.mkOption {
  type = lib.types.str;  # Opaque blob
  description = "Raw configuration string";
};

# GOOD — structured options that compose
options.myModule = {
  listenAddress = lib.mkOption { type = lib.types.str; default = "0.0.0.0"; };
  listenPort = lib.mkOption { type = lib.types.port; default = 8080; };
  extraConfig = lib.mkOption {
    type = lib.types.attrsOf lib.types.str;
    default = {};
    description = "Additional configuration key-value pairs.";
  };
};
```

---

## Rust Types for Infrastructure Tools

### Newtypes for Domain Values

Wrap primitive types in newtypes to prevent mixing up logically distinct values:

```rust
// BAD — easy to pass the wrong string in the wrong place
fn configure_peer(public_key: String, endpoint: String, psk: String) { ... }

// GOOD — newtypes make it impossible to mix up arguments
pub struct WireguardPublicKey(String);
pub struct WireguardEndpoint(String);   // "host:port"
pub struct WireguardPsk(String);

impl WireguardPublicKey {
    pub fn parse(s: &str) -> Result<Self, KeyError> {
        // validate base64 encoding and correct key length
        validate_wg_key(s)?;
        Ok(Self(s.to_owned()))
    }
}

fn configure_peer(public_key: WireguardPublicKey, endpoint: WireguardEndpoint, psk: WireguardPsk) { ... }
```

### Enumerations for State

Use enums instead of strings or booleans for values with a fixed set of options:

```rust
// BAD — stringly typed state
let mode = "full-tunnel";   // Could be anything

// GOOD — exhaustively checked by the compiler
#[derive(Debug, Clone, PartialEq)]
pub enum WireguardMode {
    HubAndSpoke,
    FullTunnel,
    SiteToSite,
    RoadWarrior,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VaultAuthMethod {
    AppRole,
    Token,
    Kubernetes,
}

impl VaultAuthMethod {
    pub fn requires_role_id(&self) -> bool {
        matches!(self, Self::AppRole)
    }
}
```

### Error Types

Define specific error types per subsystem using `thiserror`:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("Connection refused to Vault at {address}")]
    ConnectionRefused { address: String },

    #[error("Authentication failed: invalid token or role credentials")]
    AuthFailed,

    #[error("Secret not found at path: {path}")]
    SecretNotFound { path: String },

    #[error("Permission denied reading {path}: token lacks required policy")]
    PermissionDenied { path: String },

    #[error("HTTP error {status}: {message}")]
    HttpError { status: u16, message: String },

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum WireguardError {
    #[error("Invalid public key: {0}")]
    InvalidKey(String),

    #[error("Interface {interface} not found")]
    InterfaceNotFound { interface: String },

    #[error("Failed to write configuration to {path}: {source}")]
    ConfigWriteError { path: String, #[source] source: std::io::Error },
}
```

### Configuration Structs

Use derive macros and builders for configuration structures parsed from CLI or config files:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultConfig {
    pub address: String,
    pub auth_method: VaultAuthMethod,
    #[serde(skip_serializing)]  // Never serialise tokens to output
    pub token: Option<String>,
    pub role_id_path: Option<std::path::PathBuf>,
    pub secret_id_path: Option<std::path::PathBuf>,
    pub timeout_seconds: u64,
}

impl Default for VaultConfig {
    fn default() -> Self {
        Self {
            address: "https://127.0.0.1:8200".to_owned(),
            auth_method: VaultAuthMethod::AppRole,
            token: None,
            role_id_path: None,
            secret_id_path: None,
            timeout_seconds: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireguardPeer {
    pub name: String,
    pub public_key: String,
    pub allowed_ips: Vec<String>,
    pub endpoint: Option<String>,
    pub persistent_keepalive: Option<u16>,
    pub psk_vault_path: Option<String>,
}
```

---

## Configuration Data Modelling

### Device Configuration

Model device types as a Rust enum, not a string:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceType {
    Laptop,
    HomeServer,
    Router,
    CloudServer,
    Mobile,
}

impl DeviceType {
    pub fn default_profile(&self) -> &'static str {
        match self {
            Self::Laptop => "profiles/workstation.nix",
            Self::HomeServer => "profiles/server.nix",
            Self::Router => "profiles/router.nix",
            Self::CloudServer => "profiles/server.nix",
            Self::Mobile => "profiles/mobile.nix",
        }
    }

    pub fn needs_desktop(&self) -> bool {
        matches!(self, Self::Laptop | Self::Mobile)
    }
}
```

### Service Configuration

Structure service configurations to map cleanly to NixOS options:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub enable: bool,
    pub port: Option<u16>,
    pub vault_secret_paths: Vec<String>,
    pub hardening: HardeningLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HardeningLevel {
    Minimal,    // Basic: NoNewPrivileges only
    Standard,   // Standard: + ProtectSystem, PrivateTmp
    Strict,     // Maximum: + syscall filtering, capability bounding
}
```

### Wireguard Peer Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireguardNetwork {
    pub interface: String,
    pub address: String,         // CIDR notation
    pub listen_port: Option<u16>,
    pub private_key_vault_path: String,
    pub peers: Vec<WireguardPeer>,
    pub mode: WireguardMode,
}

impl WireguardNetwork {
    /// Validate the network configuration before generating NixOS config.
    pub fn validate(&self) -> Result<(), WireguardError> {
        if self.peers.is_empty() {
            return Err(WireguardError::InvalidKey("Network must have at least one peer".to_owned()));
        }
        if self.mode == WireguardMode::FullTunnel && self.listen_port.is_some() {
            // Full tunnel clients do not listen for inbound connections
            return Err(WireguardError::InvalidConfig(
                "Full tunnel mode does not use a listen port".to_owned()
            ));
        }
        Ok(())
    }
}
```

---

## Anti-Patterns

### Stringly Typed Options

```nix
# BAD — accepts any string, no validation
options.myModule.mode = lib.mkOption {
  type = lib.types.str;
  description = "Mode: 'full' or 'split'";
};

# GOOD — validated at evaluation time
options.myModule.mode = lib.mkOption {
  type = lib.types.enum [ "full" "split" ];
  description = lib.mdDoc "Tunnel mode: `full` routes all traffic, `split` routes only VPN subnet.";
};
```

### God Attribute Set

```nix
# BAD — one massive config attrset with everything in it
options.infrastructure.config = lib.mkOption {
  type = lib.types.attrs;   # Opaque blob
  description = "The entire infrastructure configuration";
};

# GOOD — separate option groups per concern
options.infrastructure = {
  wireguard = { /* ... */ };
  vault = { /* ... */ };
  services = { /* ... */ };
};
```

### Boolean Blindness

```nix
# BAD — two booleans with unclear meaning when both are false
options.myService = {
  enableSSL = lib.mkEnableOption "SSL";
  disableSSL = lib.mkOption { type = lib.types.bool; default = false; };
};

# GOOD — enum for mutually exclusive states
options.myService.tlsMode = lib.mkOption {
  type = lib.types.enum [ "disabled" "starttls" "ssl" ];
  default = "ssl";
  description = lib.mdDoc "TLS mode for service connections.";
};
```

### Nested String Interpolation

```nix
# BAD — hard to read, hard to escape correctly
systemd.services.my-service.serviceConfig.ExecStart =
  "${pkgs.foo}/bin/foo --config ${pkgs.writeText "config" ''
    addr = ${config.myModule.address}
    port = ${toString config.myModule.port}
  ''}";

# GOOD — use a separate config file derivation
let
  configFile = pkgs.writeText "my-service.conf" ''
    addr = ${config.myModule.address}
    port = ${toString config.myModule.port}
  '';
in
systemd.services.my-service.serviceConfig.ExecStart =
  "${pkgs.foo}/bin/foo --config ${configFile}";
```

### Mutable State in Nix

Nix is purely functional — there is no mutable state. Do not try to simulate it:

```nix
# BAD — Nix does not have mutable variables
let
  result = [];
  result = result ++ [ "item1" ];  # ERROR: cannot redefine 'result' in let
in result

# GOOD — build the final value directly
let
  result = [ "item1" "item2" ];
in result

# Or use lib.lists functions to transform
let
  baseItems = [ "item1" ];
  extraItems = lib.optional config.myModule.extraFeature "item2";
  result = baseItems ++ extraItems;
in result
```

---

## Rules and Principles

1. Design option schemas before writing module logic. The configuration structure drives the implementation.
2. Use `lib.types.enum` for any option with a fixed set of valid values — never `lib.types.str` with a comment listing valid values.
3. Use `lib.types.submodule` for structured nested options — never `lib.types.attrs` (opaque, no validation).
4. Every option must have `description` using `lib.mdDoc`. Options without descriptions are unusable by agents and operators.
5. Wrap primitives in Rust newtypes for domain-specific values (keys, addresses, paths). This prevents passing the wrong value in the wrong place.
6. Use Rust `enum` for any value with a finite set of states. Never use strings for internal state.
7. Error types should be per-subsystem (`VaultError`, `WireguardError`) — a single `AppError` is a `lib.types.attrs` of errors.
8. Model Nix attrsets in Rust as `HashMap<String, T>` where T is a typed struct, not `HashMap<String, serde_json::Value>`.
9. Apply the principle: if you have the right data structures, the logic becomes obvious. If the logic is complex, question the data structure.
