# Development Workflow

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Infrastructure Team
**Language:** British English (en_GB)
**Timezone:** Europe/London
**Plugin Scope:** syntek-infra (NixOS, Hyprland, Wireguard, Vault, Vaultwarden)

---

Development, deployment, and common operational tasks for this NixOS
configuration project.

---

## Getting Started

### Prerequisites

- **NixOS 24.11+** or **Nix package manager** with flakes enabled
- **Git** for version control
- **SSH key (Ed25519)** for server access and agenix secrets
- **Vault CLI** (`bao` or `vault`) if using Hashicorp Vault

### 1. Install Nix with Flakes Support

**On NixOS (24.11+):** Flakes are enabled by default. Verify:

```bash
nix --version      # Should show 2.18+
nix flake show     # Should not error
```

**On Ubuntu/Debian/macOS:**

```bash
# Install Nix
sh <(curl -L https://nixos.org/nix/install) --daemon

# Enable Flakes
mkdir -p ~/.config/nix
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
```

### 2. Clone the Repository

```bash
git clone <repository-url> nixos-config
cd nixos-config
```

### 3. Enter Development Shell

The development shell provides all required tools:

```bash
nix develop
```

**Available tools in the dev shell:**

- `nixos-rebuild` — build and deploy NixOS configurations
- `nix-tree` — visualise dependency trees
- `nix-diff` — compare Nix derivations
- `agenix` — manage encrypted secrets
- `vault` / `bao` — Vault CLI for secrets management
- `wireguard-tools` (`wg`, `wg-quick`) — Wireguard key management
- `cargo`, `rustc`, `clippy`, `rustfmt` — Rust development

### 4. Verify Setup

```bash
# Validate the flake configuration — must pass before any other work
nix flake check

# Should output: "checks passed" with no errors
```

### 5. Generate SSH Key (if needed)

Required for agenix secret management and server access:

```bash
ssh-keygen -t ed25519 -f ~/.ssh/id_nixos -C "admin@[hostname]"
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_nixos
cat ~/.ssh/id_nixos.pub  # Add this to secrets/secrets.nix
```

Store the private key passphrase in your password manager. Back it up
securely — losing this key means losing access to agenix secrets.

### 6. Configure Secrets (agenix)

Add your SSH public key to `secrets/secrets.nix`:

```nix
{
  "vault-token.age".publicKeys = [
    "ssh-ed25519 AAAAC3... admin@nixos-config"
  ];
}
```

Create or re-encrypt a secret:

```bash
agenix -e secrets/vault-token.age  # Opens $EDITOR
```

### 7. Test in a VM Before Deploying

**Always** validate configuration changes in a VM before deploying to hardware:

```bash
# Build a VM for your target host
nixos-rebuild build-vm --flake .#[hostname]

# Run the VM (requires KVM)
./result/bin/run-[hostname]-vm
```

---

## Development Workflow

### 1. Local Development Loop

```bash
# Make configuration changes in your editor

# Check the flake evaluates cleanly (fast — no build)
nix flake check --no-build

# Build the full system closure locally to catch build errors
nix build .#nixosConfigurations.[hostname].config.system.build.toplevel

# Test in a VM
nixos-rebuild build-vm --flake .#[hostname]
./result/bin/run-[hostname]-vm
```

### 2. Rust Tool Development

The Rust CLI tool (`plugins/`) follows standard Cargo workflows:

```bash
cd plugins/

# Run tests
cargo test --workspace

# Check for warnings and lint issues
cargo clippy -- -D warnings

# Format code
cargo fmt --check  # Check only (CI)
cargo fmt          # Apply formatting

# Build release binary
cargo build --release
```

### 3. Hyprland Configuration Development

Test Hyprland configuration changes in a nested session before applying:

```bash
# Run a nested Hyprland instance in the current session
Hyprland --config /path/to/test-config

# Or reload the running compositor after editing
hyprctl reload
```

Check for config syntax errors before reloading:

```bash
hyprctl --config /path/to/config dispatch noop 2>&1 | grep -i error
```

### 4. Wireguard Key Operations

```bash
# Generate a new peer key pair (store in Vault immediately)
wg genkey | tee >(vault kv put secret/wireguard/[peer]/private-key value=-) | wg pubkey

# Generate a pre-shared key for a peer pair
wg genpsk | vault kv put secret/wireguard/[peer-a]-[peer-b]/psk value=-

# Verify interface status
wg show wg0

# Check peer connectivity
wg showconf wg0
```

---

## Deployment

### Simple Deployment (nixos-rebuild)

For straightforward NixOS configurations:

```bash
# Deploy to local machine
sudo nixos-rebuild switch --flake .#[hostname]

# Deploy to remote machine (over SSH, via VPN)
nixos-rebuild switch --flake .#[hostname] \
  --target-host admin@[hostname].vpn \
  --use-remote-sudo
```

**Always test in a VM first.** Never deploy untested configuration changes to
production hardware.

### First-Time Bare Metal Deployment (nixos-anywhere)

For initial provisioning from rescue mode:

```bash
nix run github:nix-community/nixos-anywhere -- \
  --flake .#[hostname] \
  root@[server-ip]
```

This will partition disks, set up encryption, and install NixOS from the flake.

### Rolling Back a Failed Deployment

NixOS keeps previous generations. Roll back if a deployment breaks something:

```bash
# On the machine (via console or recovery)
nixos-rebuild switch --rollback

# Or choose a specific generation
nixos-rebuild switch --rollback --flake .#[hostname]
```

### Deployment Checklist

Before deploying any configuration change:

- [ ] `nix flake check` passes without errors
- [ ] Configuration tested in a VM
- [ ] Secrets updated in Vault if required
- [ ] `flake.lock` reviewed if updated
- [ ] Wireguard keys stored in Vault, not embedded in config
- [ ] User has approved the deployment (never auto-deploy)

---

## Common Tasks

### Add a New NixOS Module

1. Create the module file under `modules/`:

```bash
mkdir -p modules/[category]
$EDITOR modules/[category]/[name].nix
```

2. Import it in `configuration.nix`:

```nix
imports = [
  ./modules/[category]/[name].nix
];
```

3. Verify it evaluates:

```bash
nix flake check --no-build
```

4. Test in a VM before deploying.

### Add a New Wireguard Peer

1. Generate the key pair and store in Vault (see Wireguard key operations above).
2. Add the peer configuration to the Wireguard module:

```nix
networking.wireguard.interfaces.wg0.peers = [
  {
    publicKey = "[peer-public-key]";
    presharedKeyFile = "/run/secrets/wireguard-psk-[peer]";
    allowedIPs = [ "10.100.0.[n]/32" ];
    endpoint = "[peer-ip]:51820";  # If the peer has a static address
  }
];
```

3. Update `secrets/secrets.nix` if the peer needs access to agenix secrets.
4. Deploy to both endpoints.

### Rotate Vault Token

```bash
# Generate a new token with the same policies
vault token create -policy=[policy-name] -ttl=720h

# Update the agenix-encrypted secret
agenix -e secrets/vault-token.age

# Redeploy to pick up the new token
nixos-rebuild switch --flake .#[hostname]
```

### Update Flake Inputs

```bash
# Review what will change before updating
nix flake metadata

# Update a single input
nix flake lock --update-input nixpkgs

# Review the diff before committing
git diff flake.lock

# Update all inputs (review diff carefully)
nix flake update
```

Never update `flake.lock` without reviewing the diff. Unpinned or
auto-updated inputs are a supply-chain risk.

---

## Git Workflow

See **[CODING-PRINCIPLES.md](CODING-PRINCIPLES.md)** for the full git
standards. Summary:

- Atomic commits — one change per commit
- Conventional Commits: `feat:`, `fix:`, `refactor:`, `docs:`, `chore:`
- Never commit `result` symlinks, secrets, or `.env` files
- Never force-push to `main`

```bash
# Stage specific files only — never git add -A
git add modules/wireguard/client.nix
git add flake.lock

git commit -m "$(cat <<'EOF'
feat(wireguard): add road-warrior client module

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
EOF
)"
```

---

## Troubleshooting

### Check System Logs

```bash
# System-level logs
journalctl -b -u [service-name]

# Vault agent logs
journalctl -b -u vault-agent

# Wireguard events
journalctl -b -u wireguard-wg0

# NixOS activation errors
journalctl -b _SYSTEMD_UNIT=activate.service
```

### Check Wireguard Status

```bash
# Interface status and peer handshakes
wg show

# Show full running configuration (keys redacted in display)
wg showconf wg0

# Test connectivity through the tunnel
ping [peer-vpn-ip]
```

### Check Vault Status

```bash
# Is Vault reachable and unsealed?
vault status

# Can the current token read a secret?
vault kv get secret/wireguard/[peer]/private-key

# List accessible secrets
vault kv list secret/wireguard/
```

### Debug NixOS Module Evaluation

```bash
# See exactly what a module option evaluates to
nix eval .#nixosConfigurations.[hostname].config.networking.wireguard.interfaces

# Check the full system closure for unexpected derivations
nix-tree .#nixosConfigurations.[hostname].config.system.build.toplevel
```

### Re-run Flake Check with Verbose Output

```bash
nix flake check -v --show-trace 2>&1 | less
```

---

## Related Documentation

- **[CODING-PRINCIPLES.md](CODING-PRINCIPLES.md)** — Code standards and review checklist
- **[TESTING.md](TESTING.md)** — Test guide, patterns, and examples
- **[SECURITY.md](SECURITY.md)** — Security architecture and hardening checklist
- **[CLAUDE.md](CLAUDE.md)** — Project context, stack decisions, and agent configuration
