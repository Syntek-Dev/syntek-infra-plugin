# Templates

This directory contains project initialisation templates for different device
types.

## Available Templates

| Template                | Description                            | Skill         |
| ----------------------- | -------------------------------------- | ------------- |
| `laptop-workstation.md` | Developer laptop with Zed, Docker, VPN | device-laptop |
| `home-server.md`        | Home/business server with services     | device-server |
| `diy-router.md`         | DIY router with firewall, NAT, DNS     | device-router |
| `cloud-server.md`       | Cloud server (Hetzner/AWS/DO)          | device-server |
| `mobile-device.md`      | Pinephone/Pinetab mobile device        | device-mobile |

## Documentation Templates

These four files are copied into every project alongside the device CLAUDE.md:

| File                    | Purpose                                                  |
| ----------------------- | -------------------------------------------------------- |
| `CODING-PRINCIPLES.md`  | Code standards: Rob Pike, Linus Torvalds, error handling |
| `TESTING.md`            | Testing guide: Rust unit/integration, nixosTest, proptest |
| `SECURITY.md`           | Security: Vault, agenix, Wireguard keys, module hardening |
| `DEVELOPMENT.md`        | Workflow: dev loop, deployment, common tasks             |

## How Templates Work

When you run `/syntek-infra:init`, the agent will:

1. Ask which device type you're configuring
2. Copy the appropriate device template to your project as `.claude/CLAUDE.md`
3. Copy all four documentation files into `.claude/`
4. Help you fill in the placeholders
5. Create the initial Flake structure

## Template Structure

Each template includes:

- **Skill Target** - Which device skill to load
- **Hardware** - Device specifications to fill in
- **Profile** - Minimal vs full configuration choice
- **Services** - Checkboxes for services to enable
- **Network Configuration** - IP addresses, VPN settings
- **Placeholders** - List of values to replace

## Using Templates

### Via Command

```
/syntek-infra:init laptop
```

### Manual Copy

1. Copy the template to your project directory
2. Rename to `CLAUDE.md` or place in `.claude/CLAUDE.md`
3. Fill in all placeholders marked with `[brackets]`
4. Check/uncheck service options

## Creating Custom Templates

To create a custom template:

1. Start with the closest existing template
2. Modify for your specific needs
3. Keep the Skill Target section
4. Document all placeholders
