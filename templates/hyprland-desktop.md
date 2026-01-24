# Hyprland Desktop Configuration

**Device:** [Insert Device Name]
**Hostname:** [Insert Hostname]
**Stack:** Hyprland + NixOS
**Created:** [Insert Date]

## Skill Target

```
Skill Target: device-laptop (or custom)
```

## Hardware

- **CPU:** [Intel/AMD - specify model]
- **GPU:** [Intel/NVIDIA/AMD - specify model]
- **RAM:** [Amount in GB]
- **Monitors:** [Number of monitors]
- **Primary Display:** [Resolution @ Refresh, e.g., 2560x1440 @ 144Hz]
- **Secondary Display(s):** [Resolution @ Refresh]

## Monitor Layout

```conf
# Primary monitor (centre or main workspace)
monitor=DP-1,[Resolution]@[Refresh],[Position],1

# Secondary monitor(s)
monitor=HDMI-A-1,[Resolution]@[Refresh],[Position],1

# Example multi-monitor setup:
# monitor=DP-1,2560x1440@144,0x0,1
# monitor=HDMI-A-1,1920x1080@60,2560x0,1
```

**Layout diagram:** [Describe monitor arrangement, e.g., "Primary centre, secondary to the right"]

## Hyprland Ecosystem

Select components to install:

- [x] waybar - Status bar
- [x] wofi - Application launcher
- [x] dunst - Notifications
- [x] grim + slurp - Screenshots
- [x] wl-clipboard - Clipboard manager
- [x] hyprlock - Screen locker
- [x] hypridle - Idle management
- [x] hyprpaper - Wallpaper manager (per-monitor)
- [ ] swww - Animated wallpaper daemon

## Terminal & Shell

- [x] kitty - GPU-accelerated terminal
- [ ] alacritty - Alternative terminal
- [x] fish - Friendly shell
- [ ] zsh - Z shell

## Aesthetics

- **Animation speed:** [Fast / Medium / Slow]
- **Blur effects:** [Enabled / Disabled - disable for better performance]
- **Gaps:** [Yes / No]
- **Borders:** [Width in pixels, e.g., 2]
- **Border colours:** [Active / Inactive]
- **Opacity:** [Enabled / Disabled]
- **Shadows:** [Enabled / Disabled]

## Workspace Distribution

Define workspace-to-monitor mapping:

- **Monitor 1 (DP-1):** Workspaces 1-5
- **Monitor 2 (HDMI-A-1):** Workspaces 6-9

```conf
workspace=1,monitor:DP-1,default:true
workspace=2,monitor:DP-1
workspace=3,monitor:DP-1
workspace=4,monitor:DP-1
workspace=5,monitor:DP-1
workspace=6,monitor:HDMI-A-1,default:true
workspace=7,monitor:HDMI-A-1
workspace=8,monitor:HDMI-A-1
workspace=9,monitor:HDMI-A-1
```

## Keybinding Preferences

- **Primary modifier:** SUPER (Windows key)
- **Vim-style navigation:** [Yes / No]
- **Multi-monitor focus:** [SUPER+Mouse / SUPER+Arrow / Custom]
- **Additional modifiers:** [List any special preferences]

## Gaming Optimisations

For gaming setups:

- [ ] Disable compositor for fullscreen games
- [ ] High refresh rate on primary monitor
- [ ] Disable blur and effects for performance
- [ ] Custom window rules for games

```conf
windowrulev2 = immediate, class:^(cs2)$
windowrulev2 = immediate, class:^(steam_app_.*)$
```

## Startup Applications

List applications to auto-start:

- [ ] waybar (on all monitors)
- [ ] dunst
- [ ] hypridle
- [ ] hyprpaper (per-monitor wallpapers)
- [ ] Discord
- [ ] Steam
- [ ] Other: [Specify]

## Special Requirements

- **NVIDIA GPU:** [Yes / No - requires special env vars]
- **G-Sync/FreeSync:** [Yes / No]
- **HDR:** [Yes / No - experimental in Hyprland]
- **VRR (Variable Refresh Rate):** [Yes / No]

## NVIDIA Configuration

If using NVIDIA GPU:

```conf
env = LIBVA_DRIVER_NAME,nvidia
env = XDG_SESSION_TYPE,wayland
env = GBM_BACKEND,nvidia-drm
env = __GLX_VENDOR_LIBRARY_NAME,nvidia
env = WLR_NO_HARDWARE_CURSORS,1
```

## Placeholders to Replace

When using this template, replace:

- `[Insert Device Name]` - e.g., "Custom Gaming PC"
- `[Insert Hostname]` - e.g., "desktop-hyprland"
- `[Insert Date]` - e.g., "24/01/2026"
- Monitor names (DP-1, HDMI-A-1, etc.) - use `hyprctl monitors` to find correct names
- Monitor resolutions and refresh rates
- Workspace-to-monitor mapping
- Border colours and aesthetic preferences
