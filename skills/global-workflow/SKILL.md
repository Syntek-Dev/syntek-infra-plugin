# Global Workflow - NixOS Infrastructure

This skill is **always applied** regardless of device type. It defines standards
and conventions for all NixOS infrastructure work.

## Language & Formatting

- **British English** spelling throughout (colour, organisation, centre)
- **Date format:** DD/MM/YYYY (e.g., 22/01/2026)
- **Time format:** 24-hour clock (e.g., 14:30, not 2:30 PM)
- **Currency:** GBP where applicable

## Nix Standards

### Use NixOS Stable

Always use the stable NixOS channel, never unstable:

```nix
inputs = {
  nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";  # Stable
  # NOT: nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
};
```

### Use Nix Flakes

All configurations must use Flakes:

```nix
{
  description = "NixOS configuration";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
  };

  outputs = { self, nixpkgs }: {
    nixosConfigurations.hostname = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [ ./configuration.nix ];
    };
  };
}
```

### Pure Nix Only

- Write configurations in **pure Nix** language only
- **No JSON or YAML** configuration files
- Custom scripts may use Rust or shell

## Security Requirements

### Never Hardcode Secrets

```nix
# WRONG - Never do this
services.wireguard.privateKey = "actual-private-key-here";

# CORRECT - Use file reference
services.wireguard.privateKeyFile = "/run/secrets/wireguard-private-key";
```

### Secrets Management

Use one of these approaches:

1. **agenix** - Encrypted secrets in git, decrypted at build time
2. **sops-nix** - Alternative with SOPS encryption
3. **Runtime Vault fetch** - Fetch from Hashicorp Vault at runtime

### All Secrets via Vault

- Store all secrets in Hashicorp Vault
- Store user passwords in Vaultwarden
- Never commit secrets to git (even encrypted without agenix/sops)

## Deployment Standards

### Always Wait for User Approval

Before any deployment:

1. Show the configuration to be deployed
2. Explain what changes will be made
3. List any risks or considerations
4. **Ask for explicit confirmation**
5. Only proceed after user says yes

### Validate Before Deploy

Always validate configurations before deployment:

```bash
# Check flake syntax
nix flake check

# Dry build
nixos-rebuild dry-build --flake .#hostname

# Test in VM (when possible)
nixos-rebuild build-vm --flake .#hostname
```

### Deployment Commands

- **Local:** `sudo nixos-rebuild switch --flake .#hostname`
- **Remote:** `nixos-rebuild switch --flake .#hostname --target-host user@host`
- **Test:** `sudo nixos-rebuild test --flake .#hostname` (doesn't persist)
- **Boot:** `sudo nixos-rebuild boot --flake .#hostname` (applies on reboot)

## Code Style

### Nix Formatting

Use `nixfmt` or `alejandra` for consistent formatting:

```nix
# Good - clear structure
{
  services.nginx = {
    enable = true;
    virtualHosts."example.com" = {
      forceSSL = true;
      enableACME = true;
    };
  };
}
```

### Comments

Add comments for non-obvious configurations:

```nix
# Enable hardware acceleration for Intel GPUs
hardware.opengl = {
  enable = true;
  extraPackages = with pkgs; [
    intel-media-driver  # VAAPI for newer Intel GPUs
    vaapiIntel          # VAAPI for older Intel GPUs
  ];
};
```

## Git Standards

- Commit messages in imperative mood ("Add", "Fix", "Update")
- One logical change per commit
- Never commit secrets or private keys
- Use `.gitignore` to exclude generated files

## Error Handling

When builds fail:

1. Read the complete error message
2. Identify the failing module
3. Check for common issues (missing inputs, type errors)
4. Provide clear explanation to user
5. Suggest specific fixes
