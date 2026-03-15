---
description: '[Agent] Configure Hyprland wayland compositor'
usage: /syntek-infra:hyprland [device] [profile]
---

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

Spawn the `syntek-infra:hyprland-configurator` agent (model: sonnet) to generate
Hyprland compositor configurations.

## Pre-flight: Run Plugin Tools

Before starting Hyprland work, gather context using:

- `syntek-infra-tool hyprland detect`
- `syntek-infra-tool hyprland status`

## Reference Documents

The agent must read these files from `.claude/` before producing any configuration:

- `.claude/CODING-PRINCIPLES.md` — coding standards and review checklist
- `.claude/ARCHITECTURE-PATTERNS.md` — Hyprland configuration patterns, NixOS module structure
- `.claude/PERFORMANCE.md` — VRR/VFR settings, GPU drivers, animation tuning
- `.claude/DATA-STRUCTURES.md` — Nix option types for compositor configuration

## The Agent

The agent is a Hyprland Configuration Specialist who:

- Generates custom hyprland.conf configurations
- Configures keybindings, animations, and window rules
- Sets up multi-monitor layouts
- Integrates Hyprland ecosystem (waybar, wofi, dunst)
- Debugs Hyprland issues and optimises performance

## Required Information

If not provided, the agent will ask:

- **Device type:** laptop, desktop, multi-monitor
- **Monitor setup:** Resolution, refresh rate, scaling
- **Input devices:** Keyboard layout, touchpad settings
- **Ecosystem apps:** Terminal, launcher, bar, notifications
- **Aesthetics:** Animation preferences, gaps, borders
- **Startup apps:** Auto-start applications

## Example References

The agent uses patterns from:

- `examples/hyprland/configs/BASIC-HYPRLAND.md` - Basic configuration
- `examples/hyprland/configs/LAPTOP-HYPRLAND.md` - Laptop setup
- `examples/hyprland/configs/MULTI-MONITOR.md` - Multi-monitor configuration
- `examples/hyprland/hardware/NVIDIA-HYPRLAND.md` - NVIDIA GPU setup
- `examples/hyprland/nixos/HYPRLAND-NIXOS.md` - NixOS integration

## Usage Examples

```
/syntek-infra:hyprland
/syntek-infra:hyprland laptop
/syntek-infra:hyprland desktop minimal
/syntek-infra:hyprland multi-monitor full
```

## Configuration Structure

The agent generates a modular Hyprland configuration:

```
~/.config/hypr/
├── hyprland.conf       # Main configuration
├── monitors.conf       # Monitor settings
├── keybinds.conf       # Keybindings
├── windowrules.conf    # Window rules
├── animations.conf     # Animation settings
└── autostart.conf      # Startup applications
```

## Hyprland Ecosystem

Standard components configured alongside Hyprland:

- **waybar** - Status bar
- **wofi** - Application launcher
- **dunst** - Notifications
- **grim** + **slurp** - Screenshots
- **wl-clipboard** - Clipboard manager
- **hyprlock** - Screen locker
- **hypridle** - Idle management

## NixOS Integration

The agent also generates the NixOS configuration to install Hyprland:

```nix
programs.hyprland = {
  enable = true;
  xwayland.enable = true;
};

environment.systemPackages = with pkgs; [
  waybar
  wofi
  dunst
  grim
  slurp
  wl-clipboard
  hyprlock
  hypridle
];
```

## User's Request

$ARGUMENTS
