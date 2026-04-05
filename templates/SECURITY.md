# Security Architecture

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Infrastructure Team
**Language:** British English (en_GB)
**Timezone:** Europe/London
**Plugin Scope:** syntek-infra (NixOS, Hyprland, Wireguard, Vault, Vaultwarden)

---

Security patterns, access control, and hardening strategies for this NixOS
configuration. Read before writing any module, service, or secret configuration.

---

## Core Security Principles

**Never hardcode secrets.** All credentials, private keys, API tokens, and
passwords live in Hashicorp Vault. The only secret encrypted directly in the
repository is the Vault bootstrap token, encrypted with agenix.

**Principle of least privilege.** Every service, user, firewall rule, and Vault
policy grants only what is strictly required — nothing more.

**Defence in depth.** No single layer is the last line of defence. Wireguard
encrypts the transport. Vault controls secret access. NixOS firewall limits
ports. Each layer is independently hardened.

---

## Secrets Management

### Two-Tier Secret Strategy

1. **agenix (Bootstrap only):** Encrypts only the Vault root token or access
   credentials needed to bootstrap the server.
   - `secrets/vault-token.age` — initial Vault authentication
   - This is the **only** secret encrypted with agenix
   - Required to bootstrap the device and reach Vault

2. **Hashicorp Vault (Everything else):** All other secrets stored in Vault.
   - Wireguard private keys and pre-shared keys
   - Service credentials and API tokens
   - SSH CA signing keys
   - TLS certificates and private keys
   - Per-device encryption keys

```bash
# Bootstrap: edit Vault token (the only agenix-encrypted secret)
agenix -e secrets/vault-token.age

# All other secrets go into Vault after initial setup
vault kv put secret/wireguard/[peer-name]/private-key value="..."
```

**Never commit plaintext secrets.** The Vault token must be encrypted with
agenix before committing. Everything else goes into Vault once the device is
bootstrapped.

### Secrets Injection Pattern

Services fetch secrets from Vault at startup via systemd preStart scripts:

```nix
systemd.services."wireguard-setup" = {
  preStart = ''
    export VAULT_ADDR=${vaultAddress}
    export VAULT_TOKEN=$(cat /run/agenix/vault-token)
    vault kv get -field=value secret/wireguard/wg0/private-key \
      > /run/secrets/wireguard-private-key
    chmod 400 /run/secrets/wireguard-private-key
  '';
};
```

### Secret File Permissions

All injected secret files must be:

| Owner          | Group          | Mode  | Reason                             |
| -------------- | -------------- | ----- | ---------------------------------- |
| `root`         | `root`         | `400` | Private keys (Wireguard, SSH)      |
| `root`         | Service group  | `440` | Service credentials                |
| Service user   | Service group  | `400` | Secrets owned by specific service  |

Never use mode `644` or `666` for secret files. Never place secrets under
world-readable directories.

---

## Wireguard Security

### Key Management

- **Never store Wireguard private keys in the Nix configuration.** Keys live
  in Vault and are injected at runtime.
- Generate keys with `wg genkey` locally, store in Vault immediately, then
  destroy the local copy.
- Rotate keys on a schedule or whenever a peer is removed.

```bash
# Generate and store — never leave a plaintext key on disk
wg genkey | tee >(vault kv put secret/wireguard/[peer]/private-key value=-) | wg pubkey
```

### Pre-Shared Keys (PSKs)

Use pre-shared keys on all peer pairs for additional quantum resistance:

```nix
networking.wireguard.interfaces.wg0.peers = [
  {
    publicKey = "...";
    presharedKeyFile = "/run/secrets/wireguard-psk-[peer]";
    allowedIPs = [ "10.100.0.2/32" ];
  }
];
```

PSKs are stored in Vault alongside private keys and injected at startup.

### Firewall Rules

Only the Wireguard listen port should be exposed externally. All other inbound
traffic must be blocked at the NixOS firewall:

```nix
networking.firewall = {
  enable = true;
  allowedUDPPorts = [ 51820 ];  # Wireguard only
  allowedTCPPorts = [ ];        # No TCP exposed externally
};
```

For full-tunnel configurations, ensure the forwarding rules only permit traffic
from authenticated VPN peers:

```nix
networking.firewall.extraForwardRules = ''
  iifname "wg0" oifname "eth0" accept
  iifname "eth0" oifname "wg0" ct state { established, related } accept
'';
```

---

## NixOS Module Security

### Module Hardening Defaults

All NixOS service modules must include hardening defaults. Apply these options
to every `systemd.services.<name>` definition:

```nix
systemd.services."my-service" = {
  serviceConfig = {
    # Filesystem isolation
    ProtectSystem = "strict";
    ProtectHome = true;
    PrivateTmp = true;
    PrivateDevices = true;

    # Network isolation (remove if service needs network)
    # PrivateNetwork = true;

    # Privilege restrictions
    NoNewPrivileges = true;
    CapabilityBoundingSet = "";

    # Syscall filtering
    SystemCallFilter = [ "@system-service" ];
    SystemCallErrorNumber = "EPERM";

    # User and group
    User = "my-service";
    Group = "my-service";
    DynamicUser = true;  # Use unless persistent state is required
  };
};
```

### Immutable Module Options

Use `lib.mkAssert` to enforce invariants in module options:

```nix
{ config, lib, ... }:

{
  options.myModule.vaultAddress = lib.mkOption {
    type = lib.types.str;
    description = lib.mdDoc "Vault server address. Must use HTTPS in production.";
  };

  config = lib.mkIf config.myModule.enable {
    assertions = [
      {
        assertion = lib.hasPrefix "https://" config.myModule.vaultAddress
          || config.myModule.vaultAddress == "http://127.0.0.1:8200";
        message = "myModule.vaultAddress must use HTTPS (or 127.0.0.1 for local dev).";
      }
    ];
  };
}
```

### Firewall Module Pattern

Every module that opens a port must document why in a comment and restrict
access to the minimum required scope:

```nix
networking.firewall = {
  # SSH: only from VPN subnet — never expose to public internet
  allowedTCPPorts = lib.mkIf config.services.openssh.enable
    (lib.warn "SSH exposed — ensure firewall upstream restricts to VPN" [ 22 ]);
};
```

---

## SSH Access Control

### Key Types

Only Ed25519 keys are accepted. RSA keys are not permitted.

```nix
users.users.admin.openssh.authorizedKeys.keys = [
  "ssh-ed25519 AAAAC3... admin@hostname"
];
```

### Hardened SSH Configuration

```nix
services.openssh = {
  enable = true;
  settings = {
    PermitRootLogin = "prohibit-password";
    PasswordAuthentication = false;
    KbdInteractiveAuthentication = false;
    X11Forwarding = false;
    AllowTcpForwarding = false;
    PrintMotd = false;
  };
  # Ed25519 host keys only
  hostKeys = [
    { path = "/etc/ssh/ssh_host_ed25519_key"; type = "ed25519"; }
  ];
};
```

### VPN-Only SSH Access

For servers, restrict SSH access to the Wireguard VPN interface only:

```nix
services.openssh.listenAddresses = [
  { addr = "10.100.0.1"; port = 22; }  # VPN interface only
];
```

---

## Vault Integration Security

### Token Lifecycle

- Bootstrap tokens are short-lived (24hr) and stored only in agenix.
- Service tokens are generated from Vault roles with minimum TTLs.
- Tokens are renewed automatically via the Vault agent.
- Revoke tokens immediately on device decommission or compromise.

### Vault Policy Principle of Least Privilege

Every service gets its own Vault policy granting only the paths it needs:

```hcl
# vault-policies/wireguard.hcl
path "secret/data/wireguard/+/private-key" {
  capabilities = ["read"]
}

path "secret/data/wireguard/+/psk" {
  capabilities = ["read"]
}

# Deny everything else — implicit deny is the default, but explicit is clearer
path "*" {
  capabilities = ["deny"]
}
```

Never use `capabilities = ["create", "update", "delete", "list", "sudo"]`
unless the role explicitly requires it.

### Vault Agent Configuration

Use the Vault agent for automatic secret injection rather than scripting token
management manually:

```nix
services.vault-agent = {
  enable = true;
  settings = {
    vault.address = "https://vault.example.com";
    auto_auth = [{
      method = [{
        type = "approle";
        config = [{
          role_id_file_path = "/run/agenix/vault-role-id";
          secret_id_file_path = "/run/agenix/vault-secret-id";
        }];
      }];
    }];
  };
};
```

---

## Database Security — Row-Level Security (RLS)

RLS is required for any PostgreSQL database that holds sensitive, per-user, or
multi-tenant data. It enforces access control at the row level inside the
database engine, independent of application-layer checks.

### RLS Requirements

Every table holding sensitive data must have RLS enabled and enforced:

```sql
-- Enable RLS — no rows visible until a policy permits them
ALTER TABLE sensitive_table ENABLE ROW LEVEL SECURITY;

-- Force RLS even for the table owner
ALTER TABLE sensitive_table FORCE ROW LEVEL SECURITY;
```

A table with RLS enabled but **no matching policy returns zero rows**. This is
the correct default-deny behaviour. Never rely on application code alone to
filter rows.

### RLS Policy Design

Policies must be scoped to the PostgreSQL role, not to an ephemeral username.
Credentials are generated dynamically by the Vault database secrets engine:

```sql
-- Grant all Vault-generated credentials the PostgreSQL role for their tier
-- (done in Vault's creation_statements)
GRANT app_readonly TO "{{ vault-generated-username }}";

-- Policy references role membership — works with any dynamically named user
CREATE POLICY read_own_data ON user_records
  FOR SELECT
  USING (pg_has_role(current_user, 'app_readonly', 'USAGE'));

CREATE POLICY write_own_data ON user_records
  FOR INSERT WITH CHECK (pg_has_role(current_user, 'app_readwrite', 'USAGE'));
```

For multi-tenant data, scope policies to a session variable set by the
application at connection time:

```sql
-- Application sets current tenant on connect
SET app.current_tenant = 'tenant-id';

CREATE POLICY tenant_isolation ON tenant_data
  USING (tenant_id = current_setting('app.current_tenant')::uuid);
```

### Vault Database Secrets Engine Integration

Application services must never use static database credentials. Use the Vault
database secrets engine to generate short-lived credentials per Vault role:

```hcl
# vault-policies/app-readonly.hcl
path "database/creds/app-readonly" {
  capabilities = ["read"]
}

path "*" {
  capabilities = ["deny"]
}
```

The Vault role's `creation_statements` assign the PostgreSQL role at credential
creation time. The RLS policy then enforces data access based on that role
membership.

### PostgreSQL NixOS Module Security

```nix
services.postgresql = {
  enable = true;
  package = pkgs.postgresql_16;
  # scram-sha-256 only — never trust, md5, or password
  authentication = lib.mkForce ''
    local all postgres peer
    local all all scram-sha-256
    host  all all 127.0.0.1/32 scram-sha-256
  '';
  settings = {
    # Log all connections and DDL statements
    log_connections = true;
    log_disconnections = true;
    log_statement = "ddl";
    # Enforce SSL for network connections
    ssl = true;
  };
};
```

RLS initialisation SQL must be applied in a versioned migration, not in
`initialScript` (which only runs once). Use a migration tool (e.g. `sqitch`,
`flyway`, or `dbmate`) managed by a NixOS oneshot service.

---

## Hyprland Security Considerations

### No Remote Access

Hyprland runs as the user's compositor. Do not expose any socket or IPC
interface outside the local user session:

```nix
# Never expose Hyprland socket to other users
wayland.compositor.hyprland = {
  enable = true;
  # Run under dedicated user, not root
};
```

### Screen Locking

Always configure a screen locker. The lock must activate on suspend and after
an idle timeout:

```nix
services.hypridle = {
  enable = true;
  settings.listener = [
    { timeout = 300; on-timeout = "hyprlock"; }
    { timeout = 600; on-timeout = "systemctl suspend"; }
  ];
};
```

### Clipboard Security

Avoid clipboard managers that persist clipboard history to disk. If a clipboard
manager is required, ensure it excludes password manager entries:

```nix
# Exclude entries from password managers (e.g., KeePassXC, rbw)
services.cliphist.extraArgs = [ "--exclude" "KeePassXC" ];
```

---

## Dependency Pinning

All inputs are pinned via `flake.lock`. Never run `nix flake update` as part
of an automated process without reviewing the diff:

```bash
# Update a single input with review
nix flake lock --update-input nixpkgs
git diff flake.lock  # Review before committing
```

In Rust, all crate versions are pinned in `Cargo.lock`. Run
`cargo audit` before merging any dependency update:

```bash
cargo audit
```

---

## Security Checklist

Before marking any module or configuration complete:

- [ ] No secrets committed to git — all in Vault or agenix
- [ ] Secret files have restrictive permissions (400 or 440 maximum)
- [ ] Systemd service uses hardening options (`ProtectSystem`, `NoNewPrivileges`)
- [ ] Firewall rules open only required ports and restrict to required sources
- [ ] SSH configured for key-only auth, no root login via password
- [ ] Wireguard keys managed via Vault, not embedded in config
- [ ] Vault policies follow least-privilege (only required paths and capabilities)
- [ ] `nix flake check` passes without warnings
- [ ] All dependencies pinned (`flake.lock`, `Cargo.lock`)
- [ ] No world-readable files containing secrets or configuration values
- [ ] Screen lock configured for Hyprland desktops
- [ ] RLS enabled and forced on all PostgreSQL tables holding sensitive data
- [ ] PostgreSQL authentication uses `scram-sha-256` — no `trust` or `md5`
- [ ] Application services connect as least-privilege PostgreSQL users (never superuser)
- [ ] Database credentials generated by Vault database secrets engine, not hardcoded
- [ ] Vault RLS role policies deny all paths except `database/creds/<role>`
- [ ] RLS policies reference PostgreSQL role membership, not ephemeral usernames
- [ ] Session variables for multi-tenant isolation set on every connection
