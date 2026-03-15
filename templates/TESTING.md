# Testing Guide

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

**Purpose**: Reference document for all agents writing or reviewing tests.
Agents MUST follow these conventions, patterns, and examples.

---

## Table of Contents

- [Stack and Tooling](#stack-and-tooling)
- [Directory Structure](#directory-structure)
- [Naming Conventions](#naming-conventions)
- [The Testing Pyramid](#the-testing-pyramid)
  - [Unit Tests](#1-unit-tests)
  - [Integration Tests](#2-integration-tests)
  - [NixOS Module Tests](#3-nixos-module-tests)
  - [Smoke Tests](#4-smoke-tests)
  - [Property-Based Tests](#5-property-based-tests)
- [TDD Methodology](#tdd-test-driven-development)
- [Mocking Patterns](#mocking-patterns)
- [Async Testing](#async-testing)
- [Security-Critical Tests](#security-critical-tests)
- [Rules and Principles](#rules-and-principles)

---

## Stack and Tooling

| Tool                | Purpose                                         | How to use                                |
| ------------------- | ----------------------------------------------- | ----------------------------------------- |
| **cargo test**      | Rust unit and integration test runner           | `cargo test -p <crate>`                   |
| **tokio::test**     | Async Rust tests                                | `#[tokio::test]` attribute                |
| **mockall**         | Trait-based mocking for Rust unit tests         | `#[automock]` / `mock!` macro             |
| **wiremock**        | HTTP mock server for outbound call testing      | `MockServer::start().await`               |
| **proptest**        | Property-based testing (security-critical)      | `proptest!` macro                         |
| **assert_cmd**      | CLI integration testing                         | `Command::cargo_bin("syntek-infra-tool")` |
| **nix flake check** | Evaluate all NixOS modules for errors           | `nix flake check`                         |
| **nixosTest**       | VM-based NixOS module integration tests         | `nix build .#checks.<system>.<name>`      |

**Running Rust tests:**

```bash
# Full test suite for all crates
cargo test --workspace

# Single crate
cargo test -p syntek-infra-tool

# Specific test by name pattern
cargo test -p syntek-infra-tool vault

# With log output (useful for debugging failures)
RUST_LOG=debug cargo test -p syntek-infra-tool -- --nocapture
```

**Running NixOS checks:**

```bash
# Evaluate all modules — catches Nix syntax and type errors
nix flake check

# Build and run a specific NixOS VM test (~2–5 min, requires KVM)
nix build .#checks.x86_64-linux.<test-name>

# Run interactively with a test console
nix run .#checks.x86_64-linux.<test-name>.driver
```

---

## Directory Structure

Rust unit tests live inline as `#[cfg(test)]` modules at the bottom of each
source file. Integration tests live in a `tests/` directory at the crate root.
NixOS VM tests live in `tests/nixos/`.

```
nixos-config/
├── modules/
│   ├── wireguard/
│   │   ├── client.nix          # Unit assertions via lib.mkAssert
│   │   └── server.nix
│   ├── vault/
│   │   └── agent.nix
│   └── hyprland/
│       └── default.nix
├── tests/
│   └── nixos/
│       ├── default.nix         # Imports all NixOS module tests
│       ├── wireguard.nix       # VM test: Wireguard peer connectivity
│       └── vault-agent.nix     # VM test: Vault agent secret injection
rust-tools/
├── src/
│   ├── commands/
│   │   ├── vault.rs            # Unit tests in #[cfg(test)] at the bottom
│   │   └── wireguard.rs
│   └── lib.rs
└── tests/
    ├── cli_integration.rs      # Integration: full CLI invocation via assert_cmd
    └── vault_integration.rs    # Integration: Vault API calls via wiremock
```

---

## Naming Conventions

| Convention          | Pattern                                | Example                                              |
| ------------------- | -------------------------------------- | ---------------------------------------------------- |
| Rust unit test      | `test_<behaviour>_<condition>`         | `test_vault_connect_returns_error_on_bad_token`      |
| Rust integration    | `<module>_integration.rs`              | `vault_integration.rs`                               |
| NixOS test          | `<feature>.nix`                        | `wireguard.nix`                                      |
| Test module         | `#[cfg(test)] mod tests { ... }`       | standard Rust convention                             |
| Logical grouping    | Nested `mod` by method or behaviour    | `mod connect { ... }`                                |
| Smoke test script   | `scripts/smoke-<feature>.sh`           | `scripts/smoke-vault.sh`                             |

Avoid:

- `test_1`, `test_2`, `test_thing` — no scenario is described
- Mirroring the function name without adding scenario context:
  `test_connect_vault` is useless; `test_connect_vault_fails_on_timeout` is not

---

## The Testing Pyramid

Write tests in this ratio: many unit, some integration, few NixOS VM tests.

```
        /  NixOS VM  \      <- Few, slow, high confidence (service level)
       / Integration  \     <- Some, moderate speed (crate boundaries)
      /  Unit Tests    \    <- Many, fast, focused (function/method level)
```

### 1. Unit Tests

Test a single function or method in complete isolation. Mock all external
dependencies using `mockall`.

**What to unit test:**

- Vault client methods (token fetch, secret read/write, lease renewal)
- Wireguard config generation (peer blocks, key formatting, address assignment)
- Hyprland config serialisation (window rules, keybinds, monitor layout)
- NixOS module option validation logic
- CLI argument parsing and command dispatch
- Any pure function with clear inputs and outputs

**Example — unit test with mockall:**

```rust
// rust-tools/src/vault.rs

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[test]
    fn test_fetch_secret_returns_value_when_path_exists() {
        let mut mock = MockVaultClient::new();
        mock.expect_kv_get()
            .with(eq("secret/wireguard/peer-key"))
            .returning(|_| Ok(Some("wg-private-key-value".to_string())));

        let result = mock.kv_get("secret/wireguard/peer-key");

        assert_eq!(result.unwrap(), Some("wg-private-key-value".to_string()));
    }

    #[test]
    fn test_fetch_secret_returns_none_when_path_missing() {
        let mut mock = MockVaultClient::new();
        mock.expect_kv_get().returning(|_| Ok(None));

        let result = mock.kv_get("secret/nonexistent/path");

        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_fetch_secret_returns_error_on_connection_failure() {
        let mut mock = MockVaultClient::new();
        mock.expect_kv_get()
            .returning(|_| Err(VaultError::ConnectionRefused));

        let result = mock.kv_get("secret/any/path");

        assert!(result.is_err());
    }
}
```

**Example — pure function unit test (Wireguard config generation):**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_peer_block_includes_public_key_and_allowed_ips() {
        let peer = WireguardPeer {
            public_key: "abc123==".to_string(),
            allowed_ips: vec!["10.0.0.2/32".to_string()],
            endpoint: None,
        };

        let block = generate_peer_block(&peer);

        assert!(block.contains("PublicKey = abc123=="));
        assert!(block.contains("AllowedIPs = 10.0.0.2/32"));
    }

    #[test]
    fn test_generate_peer_block_includes_endpoint_when_set() {
        let peer = WireguardPeer {
            public_key: "abc123==".to_string(),
            allowed_ips: vec!["0.0.0.0/0".to_string()],
            endpoint: Some("vpn.example.com:51820".to_string()),
        };

        let block = generate_peer_block(&peer);

        assert!(block.contains("Endpoint = vpn.example.com:51820"));
    }
}
```

### 2. Integration Tests

Verify that multiple units work together. Integration tests live in `tests/` at
the crate root and have access to the public API only.

**What to integration test:**

- CLI commands end-to-end (parse args → execute → verify output)
- Vault client against a local test Vault instance or wiremock
- Wireguard key generation and config writing to disk
- NixOS module evaluation via `nix eval`

**Example — CLI integration test:**

```rust
// rust-tools/tests/cli_integration.rs

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_vault_status_exits_zero_with_valid_token() {
    Command::cargo_bin("syntek-infra-tool")
        .unwrap()
        .args(["vault", "status"])
        .env("VAULT_ADDR", "http://127.0.0.1:8200")
        .env("VAULT_TOKEN", "test-root-token")
        .assert()
        .success()
        .stdout(predicate::str::contains("Vault is unsealed"));
}

#[test]
fn test_wireguard_generate_rejects_missing_peer_name() {
    Command::cargo_bin("syntek-infra-tool")
        .unwrap()
        .args(["wireguard", "generate"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required argument"));
}

#[test]
fn test_nix_detect_outputs_version_information() {
    Command::cargo_bin("syntek-infra-tool")
        .unwrap()
        .args(["nix", "detect"])
        .assert()
        .success()
        .stdout(predicate::str::contains("NixOS"));
}
```

### 3. NixOS Module Tests

NixOS module tests use the `nixosTest` framework to spin up a lightweight VM,
apply the module configuration, and verify the running system. These tests are
slow but provide high confidence that a module works end-to-end.

**What to NixOS test:**

- Wireguard interface comes up and peers are reachable
- Vault agent starts and injects secrets into the expected paths
- Hyprland starts without fatal errors (in a headless VIRT display)
- Firewall rules block ports that should be blocked

**Example — NixOS VM test for Vault agent:**

```nix
# tests/nixos/vault-agent.nix

{ pkgs, ... }:

pkgs.nixosTest {
  name = "vault-agent-secret-injection";

  nodes.server = { config, pkgs, ... }: {
    imports = [
      ../../modules/vault/agent.nix
    ];

    # Stub Vault address for the test environment
    services.vault-agent.vaultAddress = "http://127.0.0.1:8200";
    services.vault-agent.enable = true;
  };

  testScript = ''
    server.start()
    server.wait_for_unit("vault-agent.service")

    # Secret must be written to the expected path
    server.succeed(
      "test -f /run/secrets/wireguard-private-key"
    )

    # Secret file must not be world-readable
    server.fail(
      "stat -c '%a' /run/secrets/wireguard-private-key | grep -q '.*[0-9][4-7]$'"
    )

    # Agent must restart cleanly without losing secrets
    server.succeed("systemctl restart vault-agent.service")
    server.sleep(3)
    server.succeed("test -f /run/secrets/wireguard-private-key")
  '';
}
```

**Example — NixOS VM test for Wireguard:**

```nix
# tests/nixos/wireguard.nix

{ pkgs, ... }:

pkgs.nixosTest {
  name = "wireguard-peer-connectivity";

  nodes = {
    server = { config, pkgs, ... }: {
      imports = [ ../../modules/wireguard/server.nix ];
      networking.wireguard.interfaces.wg0 = {
        ips = [ "10.100.0.1/24" ];
        listenPort = 51820;
        # Private key from test stub — never a real key
        privateKeyFile = ./stubs/server-private-key;
      };
    };

    client = { config, pkgs, ... }: {
      imports = [ ../../modules/wireguard/client.nix ];
      networking.wireguard.interfaces.wg0 = {
        ips = [ "10.100.0.2/24" ];
        privateKeyFile = ./stubs/client-private-key;
      };
    };
  };

  testScript = ''
    server.start()
    client.start()
    server.wait_for_unit("wireguard-wg0.service")
    client.wait_for_unit("wireguard-wg0.service")

    # Client must reach the server over the VPN tunnel
    client.succeed("ping -c 3 10.100.0.1")

    # VPN port must NOT be reachable from outside the tunnel
    server.fail("curl --max-time 3 http://10.100.0.2:8080 2>/dev/null")
  '';
}
```

**Running NixOS tests:**

```bash
# Build and run (requires KVM, takes ~2–5 minutes)
nix build .#checks.x86_64-linux.vault-agent-secret-injection

# Interactive test console (allows manual inspection)
nix run .#checks.x86_64-linux.vault-agent-secret-injection.driver
```

### 4. Smoke Tests

Quick sanity checks run after every deployment to verify core service paths
respond. Shell scripts, not automated test suites.

**Example — post-deployment smoke test:**

```bash
#!/usr/bin/env bash
# scripts/smoke-test.sh
# Run after every nixos-rebuild switch to verify core services are up.

set -euo pipefail

check() {
  local desc="$1" cmd="$2" expected="$3"
  if eval "$cmd" 2>/dev/null | grep -q "$expected"; then
    echo "PASS: $desc"
  else
    echo "FAIL: $desc — expected '$expected'" >&2
    exit 1
  fi
}

check "Wireguard interface up"  "ip link show wg0"              "UP"
check "Vault agent running"     "systemctl is-active vault-agent" "active"
check "Nix channel reachable"   "nix-channel --list"            "nixos"
check "Flake evaluates cleanly" "nix flake check --no-build"    "done"
```

### 5. Property-Based Tests

Use `proptest` for security-critical functions where manual test cases cannot
cover the full input space. Required for input validation, parsing, key
generation, and any function that processes external or user-controlled data.

**Example — property-based test for config parsing:**

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_parse_wireguard_key_never_panics_on_arbitrary_input(input in ".*") {
        // Must return Err gracefully, never panic
        let _ = parse_wireguard_key(&input);
    }

    #[test]
    fn test_vault_path_normalisation_never_escapes_prefix(
        prefix in "[a-z/]{1,32}",
        suffix in ".*"
    ) {
        let normalised = normalise_vault_path(&prefix, &suffix);
        prop_assert!(normalised.starts_with(&prefix));
    }

    #[test]
    fn test_sanitise_nix_string_never_contains_injection_sequences(
        input in ".*"
    ) {
        let sanitised = sanitise_nix_string(&input);
        // Nix string interpolation sequences must not survive sanitisation
        prop_assert!(!sanitised.contains("${"));
        prop_assert!(!sanitised.contains("''"));
    }
}
```

---

## TDD (Test-Driven Development)

**Cycle:** Red → Green → Refactor

1. **Red** — Write a failing test for the next piece of behaviour.
2. **Green** — Write the minimum code to make it pass.
3. **Refactor** — Clean up without breaking the test.

**Use TDD for:**

- All Rust CLI command handlers
- Vault client methods
- Wireguard config generation functions
- Hyprland config serialisation
- Any pure function with clear inputs and outputs

---

## Mocking Patterns

### Trait-based mocking with mockall

Define a trait for every external dependency and annotate it with `#[automock]`:

```rust
use mockall::automock;

#[automock]
pub trait VaultClient {
    fn kv_get(&self, path: &str) -> Result<Option<String>, VaultError>;
    fn kv_put(&self, path: &str, value: &str) -> Result<(), VaultError>;
    fn token_renew(&self) -> Result<(), VaultError>;
}
```

### HTTP mocking with wiremock

For services that make outbound HTTP calls (Vault API, Mullvad API), use
`wiremock` to spin up a local mock server:

```rust
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path, header};

#[tokio::test]
async fn test_vault_kv_get_sends_correct_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/secret/data/wireguard/peer-key"))
        .and(header("X-Vault-Token", "test-token"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "data": { "data": { "value": "wg-private-key" } }
            })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = VaultHttpClient::new(&mock_server.uri(), "test-token");
    let result = client.kv_get("wireguard/peer-key").await.unwrap();

    assert_eq!(result, Some("wg-private-key".to_string()));
    mock_server.verify().await;
}
```

### General mocking rules

- Mock at the boundary closest to the unit under test.
- Never mock the module you are testing.
- Use `mockall` for trait-based mocking; use `wiremock` for HTTP boundaries.
- Always verify mock expectations.
- Each test creates its own fresh mock instances — no shared mock state.

---

## Async Testing

Use `#[tokio::test]` for all async tests. Never use `block_on` inside a test.

```rust
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_vault_fetch_completes_within_deadline() {
    let client = build_test_client().await;

    let result = timeout(
        Duration::from_secs(2),
        client.kv_get("secret/test/key"),
    )
    .await;

    assert!(result.is_ok(), "vault fetch exceeded 2s deadline");
}
```

---

## Security-Critical Tests

For key parsing, config sanitisation, and authentication logic, use `proptest`
in addition to manual test cases. Manual cases alone cannot cover the full input
space for security-critical code.

**Required for:**

- Wireguard key parsing and validation
- Vault token handling and path construction
- Any function that processes external or user-controlled strings
- Nix string sanitisation (prevent expression injection)

---

## Rules and Principles

1. **Every new public Rust function has at least one unit test.** No exceptions.

2. **Every new CLI command has integration tests** covering at minimum: the
   happy path, a missing required argument, and an invalid value.

3. **Every new NixOS module passes `nix flake check`** before being merged.
   Modules with runtime behaviour have a `nixosTest` smoke check.

4. **Tests must be deterministic.** No reliance on real time, random values, or
   external network services. Mock everything at the boundary.

5. **Tests must be independent.** Each test sets up its own state and cleans
   up after itself. No test depends on another having run first.

6. **Follow Arrange-Act-Assert:**
   - **Arrange**: set up test data and mocks
   - **Act**: call the function or trigger the action
   - **Assert**: verify the outcome

7. **Test behaviour, not implementation.** Assert on outputs and observable
   side effects, not on which internal methods were called.

8. **Keep unit tests fast.** Rust unit tests should complete in under 10ms
   each. If a test needs real network or disk I/O, it belongs in the
   integration tests directory.

9. **Security-critical functions use proptest** in addition to manual test
   cases. Manual cases alone do not cover the full input space.

10. **Test code is held to the same standard as production code.** A clear,
    slightly repetitive test is better than a clever abstraction that obscures
    what is being verified.
