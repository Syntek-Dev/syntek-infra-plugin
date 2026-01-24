# Commands

This directory contains slash command definitions for the syntek-infra plugin.

## Available Commands

| Command                   | Description                         | Agent            |
| ------------------------- | ----------------------------------- | ---------------- |
| `/syntek-infra:nixos`     | Generate NixOS configuration        | nixos-builder    |
| `/syntek-infra:wireguard` | Configure Wireguard VPN             | network-engineer |
| `/syntek-infra:secrets`   | Manage secrets in Vault/Vaultwarden | vault-manager    |
| `/syntek-infra:deploy`    | Deploy NixOS configuration          | infra-architect  |
| `/syntek-infra:init`      | Initialise a new NixOS project      | infra-architect  |

## Usage Examples

### Generate NixOS Configuration

```
/syntek-infra:nixos laptop full
```

### Configure Wireguard VPN

```
/syntek-infra:wireguard outbound-vpn
/syntek-infra:wireguard home-access phone
```

### Manage Secrets

```
/syntek-infra:secrets status
/syntek-infra:secrets rotate wireguard
```

### Deploy Configuration

```
/syntek-infra:deploy laptop local
/syntek-infra:deploy server 192.168.1.10
```

### Initialise New Project

```
/syntek-infra:init laptop
```

## Command Arguments

Commands accept positional arguments that are passed to the agent via
`$ARGUMENTS`. If arguments are not provided, the agent will ask interactively.
