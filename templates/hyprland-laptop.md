# Hyprland Laptop Configuration

**Device:** [Insert Device Name]
**Hostname:** [Insert Hostname]
**Stack:** Hyprland + NixOS
**Created:** [Insert Date]

## Skill Target

```
Skill Target: device-laptop
```

## Hardware

- **CPU:** [Intel/AMD - specify model]
- **GPU:** [Intel/NVIDIA/AMD - specify model]
- **RAM:** [Amount in GB]
- **Display:** [Resolution, e.g., 2880x1800 @ 90Hz]
- **Scaling:** [1.0 / 1.5 / 2.0 for HiDPI]
- **Touchpad:** [Yes/No]

## Hyprland Ecosystem

Select components to install:

- [x] waybar - Status bar
- [x] wofi - Application launcher
- [x] dunst - Notifications
- [x] grim + slurp - Screenshots
- [x] wl-clipboard - Clipboard manager
- [x] hyprlock - Screen locker
- [x] hypridle - Idle management
- [x] hyprpaper - Wallpaper manager
- [ ] swayosd - On-screen display for volume/brightness

## Terminal & Shell

- [x] kitty - GPU-accelerated terminal
- [ ] alacritty - Alternative terminal
- [x] fish - Friendly shell
- [ ] zsh - Z shell

## Aesthetics

- **Animation speed:** [Fast / Medium / Slow]
- **Blur effects:** [Enabled / Disabled]
- **Gaps:** [Yes / No]
- **Borders:** [Width in pixels, e.g., 2]
- **Border colours:** [Active / Inactive]
- **Opacity:** [Enabled / Disabled]

## Monitor Configuration

```conf
# Primary laptop display
monitor=eDP-1,[Resolution]@[Refresh],[Position],[Scale]
# Example: monitor=eDP-1,2880x1800@90,0x0,1.5
```

## Keybinding Preferences

- **Primary modifier:** SUPER (Windows key)
- **Vim-style navigation:** [Yes / No]
- **Additional modifiers:** [List any special preferences]

## Power Management

- **Idle timeout:** [300 seconds / 5 minutes]
- **Screen lock on suspend:** [Yes / No]
- **Dim screen before lock:** [Yes / No]

## Startup Applications

List applications to auto-start:

- [ ] waybar
- [ ] dunst
- [ ] hypridle
- [ ] hyprpaper
- [ ] nm-applet (NetworkManager)
- [ ] blueman-applet (Bluetooth)
- [ ] Other: [Specify]

## Special Requirements

- **NVIDIA GPU:** [Yes / No - requires special env vars]
- **HiDPI:** [Yes / No]
- **Multi-monitor:** [Yes / No]
- **Gaming optimisations:** [Yes / No]

## Placeholders to Replace

When using this template, replace:

- `[Insert Device Name]` - e.g., "Dell XPS 15 9520"
- `[Insert Hostname]` - e.g., "laptop-hyprland"
- `[Insert Date]` - e.g., "24/01/2026"
- Monitor resolution and refresh rate
- Scaling factor for HiDPI
- Border colours and aesthetic preferences
