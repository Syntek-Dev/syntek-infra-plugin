# Secrets Injection with agenix

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

Using agenix to encrypt secrets in git and decrypt at build time.

## Use Case

Store encrypted secrets in your git repository that are:

- Safe to commit (encrypted with age)
- Decrypted automatically during NixOS build
- Available at specific paths for services

## Prerequisites

1. Install age: `nix-env -iA nixpkgs.age`
2. Generate age key or use SSH key
3. Add agenix to your flake

## Setup

### flake.nix

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";

    agenix = {
      url = "github:ryantm/agenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, agenix, ... }: {
    nixosConfigurations.my-host = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ./configuration.nix
        agenix.nixosModules.default
      ];
    };
  };
}
```

### secrets/secrets.nix

Define which keys can decrypt which secrets:

```nix
let
  # Your age public key (or SSH public key)
  myKey = "age1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";

  # Or use SSH key
  # myKey = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI...";

  # Host keys (from /etc/ssh/ssh_host_ed25519_key.pub)
  laptop = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI... root@laptop";
  server = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI... root@server";

  # Systems that can decrypt each secret
  allSystems = [ laptop server myKey ];
  laptopOnly = [ laptop myKey ];
  serverOnly = [ server myKey ];
in
{
  # Wireguard keys
  "wireguard-laptop.age".publicKeys = laptopOnly;
  "wireguard-server.age".publicKeys = serverOnly;
  "wireguard-phone.age".publicKeys = serverOnly;  # Server generates phone config

  # Service secrets
  "database-password.age".publicKeys = serverOnly;
  "vault-token.age".publicKeys = allSystems;

  # User passwords
  "user-password-hash.age".publicKeys = allSystems;
}
```

### Creating Encrypted Secrets

```bash
# Navigate to secrets directory
cd secrets

# Create and encrypt a secret
echo "my-wireguard-private-key" | agenix -e wireguard-laptop.age

# Or edit interactively
agenix -e wireguard-laptop.age

# Re-encrypt all secrets (after adding new keys to secrets.nix)
agenix -r
```

### Directory Structure

```
nixos-config/
├── flake.nix
├── configuration.nix
└── secrets/
    ├── secrets.nix           # Key definitions
    ├── wireguard-laptop.age  # Encrypted secrets
    ├── wireguard-server.age
    ├── database-password.age
    └── vault-token.age
```

## Using Secrets in Configuration

### configuration.nix

```nix
{ config, pkgs, ... }:

{
  # Declare secrets
  age.secrets = {
    wireguard-key = {
      file = ./secrets/wireguard-laptop.age;
      path = "/run/secrets/wireguard-private-key";
      owner = "root";
      group = "root";
      mode = "0400";
    };

    vault-token = {
      file = ./secrets/vault-token.age;
      path = "/run/secrets/vault-token";
      mode = "0400";
    };

    # For a specific service user
    database-password = {
      file = ./secrets/database-password.age;
      owner = "postgres";
      group = "postgres";
      mode = "0400";
    };
  };

  # Use in Wireguard
  networking.wireguard.interfaces.wg0 = {
    privateKeyFile = config.age.secrets.wireguard-key.path;
    # ... rest of config
  };

  # Use in systemd service
  systemd.services.my-service = {
    serviceConfig = {
      EnvironmentFile = config.age.secrets.vault-token.path;
    };
  };
}
```

### With User Password

```nix
{
  age.secrets.user-password = {
    file = ./secrets/user-password-hash.age;
  };

  users.users.myuser = {
    isNormalUser = true;
    hashedPasswordFile = config.age.secrets.user-password.path;
  };
}
```

## Generating Secrets

### Wireguard Key

```bash
# Generate key pair
wg genkey > /tmp/private
wg pubkey < /tmp/private > /tmp/public

# Encrypt private key
cat /tmp/private | agenix -e secrets/wireguard-laptop.age

# Save public key somewhere (not secret)
cat /tmp/public
# Add to your config or Vault

# Clean up
rm /tmp/private /tmp/public
```

### Password Hash

```bash
# Generate password hash
mkpasswd -m sha-512 "my-password" | agenix -e secrets/user-password-hash.age
```

## Retrieving Host SSH Keys

To add a new host to secrets.nix, get its SSH public key:

```bash
# On the host
cat /etc/ssh/ssh_host_ed25519_key.pub

# Or remotely
ssh-keyscan -t ed25519 hostname 2>/dev/null | cut -d' ' -f2-
```

## Best Practices

1. **Never commit unencrypted secrets**
2. **Use separate keys for different security levels**
3. **Keep your age private key secure** (not in git)
4. **Rotate secrets regularly**
5. **Use mode 0400** for sensitive files
6. **Specify owner/group** when needed by services
