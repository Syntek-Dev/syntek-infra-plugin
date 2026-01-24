# Skills

This directory contains device-specific skills that provide contextual guidance
for NixOS configurations.

## Available Skills

| Skill             | Description                            | Auto-Load |
| ----------------- | -------------------------------------- | --------- |
| `global-workflow` | Standards and conventions for all work | Yes       |
| `device-laptop`   | Developer laptop configurations        | No        |
| `device-server`   | Home/business server configurations    | No        |
| `device-router`   | DIY router configurations              | No        |
| `device-mobile`   | Pinephone/Pinetab configurations       | No        |

## How Skills Work

Skills are loaded based on the `Skill Target` in the project's `CLAUDE.md`:

```markdown
## Skill Target
```

Skill Target: device-laptop

```

```

The `global-workflow` skill is **always loaded** regardless of device type.

## Skill Contents

Each skill provides:

- **Hardware considerations** - Device-specific hardware configuration
- **Service configuration** - Common services for that device type
- **Wireguard patterns** - VPN configurations appropriate for the device
- **Profile options** - Minimal vs full configurations
- **Security recommendations** - Device-specific security hardening

## Adding New Skills

To add a new device skill:

1. Create directory: `skills/device-{name}/`
2. Create `SKILL.md` with device-specific guidance
3. Update `plugin.json` to register the skill
4. Update this README
