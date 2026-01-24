# Secrets Management with sops-nix

Using sops-nix to encrypt secrets with age or GPG.

## Use Case

sops-nix provides an alternative to agenix with:

- YAML/JSON/ENV file format support
- Multiple encryption backends (age, GPG, cloud KMS)
- Editor-friendly workflow (decrypt in place)
- Integration with Mozilla SOPS

## Setup

### flake.nix

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";

    sops-nix = {
      url = "github:Mic92/sops-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, sops-nix, ... }: {
    nixosConfigurations.my-host = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        sops-nix.nixosModules.sops
        ./configuration.nix
      ];
    };
  };
}
```

### Generate Age Key

```bash
# Generate age key for your user
mkdir -p ~/.config/sops/age
age-keygen -o ~/.config/sops/age/keys.txt

# Note the public key output, e.g.:
# public key: age1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx

# For host key (use SSH host key)
ssh-keygen -t ed25519 -f /etc/ssh/ssh_host_ed25519_key -N ""
# Or convert existing SSH key to age:
nix-shell -p ssh-to-age --run 'ssh-to-age < /etc/ssh/ssh_host_ed25519_key.pub'
```

### .sops.yaml Configuration

Create `.sops.yaml` in your repository root:

```yaml
# .sops.yaml
keys:
  # Your personal age key
  - &admin age1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx

  # Host keys (converted from SSH)
  - &server-host age1yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
  - &laptop-host age1zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz

creation_rules:
  # Secrets for all hosts
  - path_regex: secrets/common/.*\.yaml$
    key_groups:
      - age:
          - *admin
          - *server-host
          - *laptop-host

  # Server-only secrets
  - path_regex: secrets/server/.*\.yaml$
    key_groups:
      - age:
          - *admin
          - *server-host

  # Laptop-only secrets
  - path_regex: secrets/laptop/.*\.yaml$
    key_groups:
      - age:
          - *admin
          - *laptop-host
```

## Creating Secrets

### Secrets File Structure

```bash
# Create secrets directory
mkdir -p secrets/common secrets/server secrets/laptop

# Create and encrypt a secrets file
sops secrets/common/wireguard.yaml
```

### secrets/common/wireguard.yaml

```yaml
# This file is encrypted by SOPS
wireguard:
  private_key: YOUR_PRIVATE_KEY_HERE
  preshared_key: YOUR_PRESHARED_KEY_HERE

# Additional secrets in same file
api_keys:
  github: ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
  docker_hub: dckr_pat_xxxxxxxxxxxxxxxxxxxxxxxx
```

### Edit Existing Secrets

```bash
# Edit encrypted file (decrypts in memory, re-encrypts on save)
sops secrets/common/wireguard.yaml

# View without editing
sops -d secrets/common/wireguard.yaml
```

## Using Secrets in NixOS

### configuration.nix

```nix
{ config, pkgs, ... }:

{
  # Configure sops
  sops = {
    # Default sops file for this host
    defaultSopsFile = ./secrets/common/wireguard.yaml;

    # Age key for decryption (uses SSH host key)
    age = {
      # Use SSH host key
      sshKeyPaths = [ "/etc/ssh/ssh_host_ed25519_key" ];

      # Or use a dedicated age key
      # keyFile = "/var/lib/sops-nix/key.txt";
      # generateKey = true;
    };

    # Declare secrets
    secrets = {
      # Simple secret
      "wireguard/private_key" = {
        owner = "root";
        group = "root";
        mode = "0400";
      };

      # Secret with custom path
      "wireguard/preshared_key" = {
        path = "/run/secrets/wg-psk";
        owner = "root";
        mode = "0400";
      };

      # Secret from different file
      "api_keys/github" = {
        sopsFile = ./secrets/common/wireguard.yaml;
        owner = "myuser";
        mode = "0400";
      };
    };
  };

  # Use in Wireguard config
  networking.wireguard.interfaces.wg0 = {
    privateKeyFile = config.sops.secrets."wireguard/private_key".path;

    peers = [{
      publicKey = "PEER_PUBLIC_KEY";
      presharedKeyFile = config.sops.secrets."wireguard/preshared_key".path;
      allowedIPs = [ "10.100.0.0/24" ];
    }];
  };
}
```

### Multiple Sops Files

```nix
{ config, pkgs, ... }:

{
  sops = {
    age.sshKeyPaths = [ "/etc/ssh/ssh_host_ed25519_key" ];

    secrets = {
      # From default file
      "wireguard/private_key" = {
        sopsFile = ./secrets/common/wireguard.yaml;
      };

      # From server-specific file
      "database/password" = {
        sopsFile = ./secrets/server/database.yaml;
        owner = "postgres";
      };

      # From another file
      "smtp/password" = {
        sopsFile = ./secrets/server/mail.yaml;
      };
    };
  };
}
```

### Binary Secrets

For binary secrets (certificates, keys):

```nix
{
  sops.secrets."certificates/server.crt" = {
    format = "binary";
    sopsFile = ./secrets/server/server.crt.enc;
  };
}
```

Create binary secret:

```bash
sops -e --input-type binary --output-type binary \
  server.crt > secrets/server/server.crt.enc
```

## Templates

Generate config files with secrets:

```nix
{ config, pkgs, ... }:

{
  sops = {
    secrets."db_password" = { };
    secrets."api_key" = { };

    # Template that combines multiple secrets
    templates."app-config.env" = {
      content = ''
        DATABASE_URL=postgres://user:${config.sops.placeholder."db_password"}@localhost/mydb
        API_KEY=${config.sops.placeholder."api_key"}
        NODE_ENV=production
      '';

      path = "/run/secrets/app-config.env";
      owner = "myapp";
      mode = "0400";
    };
  };

  systemd.services.myapp = {
    serviceConfig = {
      EnvironmentFile = config.sops.templates."app-config.env".path;
    };
  };
}
```

## User Secrets (Home Manager)

```nix
{ config, pkgs, ... }:

{
  # In home-manager configuration
  sops = {
    age.keyFile = "/home/myuser/.config/sops/age/keys.txt";

    secrets = {
      "ssh/id_ed25519" = {
        sopsFile = ./secrets/user/ssh.yaml;
        path = "/home/myuser/.ssh/id_ed25519";
        mode = "0600";
      };

      "git/credentials" = {
        sopsFile = ./secrets/user/git.yaml;
      };
    };
  };
}
```

## Directory Structure

```
nixos-config/
├── flake.nix
├── configuration.nix
├── .sops.yaml                    # SOPS configuration
└── secrets/
    ├── common/                   # Shared secrets
    │   ├── wireguard.yaml
    │   └── api-keys.yaml
    ├── server/                   # Server-only secrets
    │   ├── database.yaml
    │   └── certificates/
    │       └── server.crt.enc
    └── laptop/                   # Laptop-only secrets
        └── user.yaml
```

## Commands Reference

```bash
# Create new encrypted file
sops secrets/new-file.yaml

# Edit existing file
sops secrets/existing.yaml

# Decrypt to stdout
sops -d secrets/file.yaml

# Encrypt existing file
sops -e plaintext.yaml > secrets/encrypted.yaml

# Rotate keys (after updating .sops.yaml)
sops updatekeys secrets/file.yaml

# Rotate all files
find secrets -name "*.yaml" -exec sops updatekeys {} \;

# Use specific key file
SOPS_AGE_KEY_FILE=~/.config/sops/age/keys.txt sops secrets/file.yaml
```

## Comparison: sops-nix vs agenix

| Feature                   | sops-nix            | agenix                |
| ------------------------- | ------------------- | --------------------- |
| File format               | YAML/JSON/ENV/INI   | Single value per file |
| Encryption                | age, GPG, cloud KMS | age only              |
| Edit workflow             | In-place edit       | Pipe-based            |
| Multiple secrets per file | Yes                 | No                    |
| Templates                 | Yes                 | No                    |
| Complexity                | Higher              | Lower                 |

Choose sops-nix when you have many secrets or need templates. Choose agenix for
simpler setups.
