# Security Architecture

**Last Updated:** [Insert Date]

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
