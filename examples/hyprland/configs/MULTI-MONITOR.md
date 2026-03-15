# Multi-Monitor Hyprland Configuration

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

Configuration for desktop setups with multiple monitors.

## Key Features

- Per-monitor workspace assignment
- Monitor-specific wallpapers
- Seamless window movement between monitors
- Independent workspace switching per monitor
- Gaming optimisations for primary monitor

## Monitor Detection

First, detect your monitors:

```bash
hyprctl monitors
```

Example output:
```
Monitor DP-1 (ID 0):
  2560x1440@143.97Hz at 0x0
  ...

Monitor HDMI-A-1 (ID 1):
  1920x1080@60.00Hz at 2560x0
  ...
```

## hyprland.conf

```conf
# Hyprland Multi-Monitor Configuration

# Monitor Configuration
# Primary monitor (high refresh rate gaming monitor)
monitor=DP-1,2560x1440@144,0x0,1

# Secondary monitor (standard office monitor)
monitor=HDMI-A-1,1920x1080@60,2560x0,1

# If you have a third monitor:
# monitor=HDMI-A-2,1920x1080@60,4480x0,1

# Fallback for any unspecified monitors
monitor=,preferred,auto,1

# Execute at launch
exec-once = waybar  # Automatically spawns on all monitors
exec-once = dunst
exec-once = hyprpaper  # Per-monitor wallpapers
exec-once = hypridle

# Environment variables
env = XCURSOR_SIZE,24
env = QT_QPA_PLATFORMTHEME,qt5ct

# Input configuration
input {
    kb_layout = gb
    kb_variant =
    kb_model =
    kb_options =
    kb_rules =

    follow_mouse = 1
    mouse_refocus = false  # Don't auto-focus on mouse movement between monitors

    sensitivity = 0
}

# General window configuration
general {
    gaps_in = 5
    gaps_out = 10
    border_size = 2
    col.active_border = rgba(33ccffee) rgba(00ff99ee) 45deg
    col.inactive_border = rgba(595959aa)

    layout = dwindle

    allow_tearing = false
}

# Decoration
decoration {
    rounding = 8

    blur {
        enabled = true
        size = 3
        passes = 1
        vibrancy = 0.1696
    }

    drop_shadow = true
    shadow_range = 4
    shadow_render_power = 3
    col.shadow = rgba(1a1a1aee)
}

# Animations
animations {
    enabled = true
    bezier = myBezier, 0.05, 0.9, 0.1, 1.05

    animation = windows, 1, 7, myBezier
    animation = windowsOut, 1, 7, default, popin 80%
    animation = border, 1, 10, default
    animation = fade, 1, 7, default
    animation = workspaces, 1, 6, default
}

# Layouts
dwindle {
    pseudotile = true
    preserve_split = true
}

master {
    new_is_master = true
}

# Miscellaneous
misc {
    force_default_wallpaper = 0
    disable_hyprland_logo = true
    vfr = false  # Disable VFR for gaming setups
}

# ===== WORKSPACE CONFIGURATION =====

# Assign workspaces to monitors
# Primary monitor (DP-1): Workspaces 1-5
workspace = 1, monitor:DP-1, default:true
workspace = 2, monitor:DP-1
workspace = 3, monitor:DP-1
workspace = 4, monitor:DP-1
workspace = 5, monitor:DP-1

# Secondary monitor (HDMI-A-1): Workspaces 6-9
workspace = 6, monitor:HDMI-A-1, default:true
workspace = 7, monitor:HDMI-A-1
workspace = 8, monitor:HDMI-A-1
workspace = 9, monitor:HDMI-A-1

# Special workspace (scratchpad) - follows focus
workspace = special:scratchpad, on-created-empty:kitty

# ===== WINDOW RULES =====

# Float windows
windowrulev2 = float, class:^(pavucontrol)$
windowrulev2 = float, class:^(nm-connection-editor)$
windowrulev2 = float, class:^(blueman-manager)$

# Force specific applications to specific monitors
# Example: Force Discord to secondary monitor
windowrulev2 = workspace 6 silent, class:^(discord)$

# Example: Force Steam to secondary monitor
windowrulev2 = workspace 7 silent, class:^(steam)$

# Gaming optimisations - disable compositor for fullscreen games
windowrulev2 = immediate, class:^(cs2)$
windowrulev2 = immediate, class:^(steam_app_.*)$
windowrulev2 = immediate, fullscreen:1

# Opacity rules
windowrulev2 = opacity 0.95 0.85, class:^(kitty)$
windowrulev2 = opacity 0.95 0.85, class:^(Code)$

# ===== KEYBINDINGS =====

$mainMod = SUPER

# Application bindings
bind = $mainMod, Q, exec, kitty
bind = $mainMod, C, killactive,
bind = $mainMod, M, exit,
bind = $mainMod, E, exec, thunar
bind = $mainMod, V, togglefloating,
bind = $mainMod, R, exec, wofi --show drun
bind = $mainMod, F, fullscreen, 0
bind = $mainMod, P, pseudo,
bind = $mainMod, J, togglesplit,

# Scratchpad (special workspace)
bind = $mainMod, S, togglespecialworkspace, scratchpad
bind = $mainMod SHIFT, S, movetoworkspace, special:scratchpad

# Move focus with vim keys
bind = $mainMod, h, movefocus, l
bind = $mainMod, l, movefocus, r
bind = $mainMod, k, movefocus, u
bind = $mainMod, j, movefocus, d

# Move windows with vim keys + shift
bind = $mainMod SHIFT, h, movewindow, l
bind = $mainMod SHIFT, l, movewindow, r
bind = $mainMod SHIFT, k, movewindow, u
bind = $mainMod SHIFT, j, movewindow, d

# Resize windows with vim keys + ctrl
bind = $mainMod CTRL, h, resizeactive, -40 0
bind = $mainMod CTRL, l, resizeactive, 40 0
bind = $mainMod CTRL, k, resizeactive, 0 -40
bind = $mainMod CTRL, j, resizeactive, 0 40

# Focus monitor with mainMod + comma/period
bind = $mainMod, comma, focusmonitor, -1
bind = $mainMod, period, focusmonitor, +1

# Move window to other monitor
bind = $mainMod SHIFT, comma, movewindow, mon:-1
bind = $mainMod SHIFT, period, movewindow, mon:+1

# Move workspace to other monitor
bind = $mainMod CTRL SHIFT, comma, movecurrentworkspacetomonitor, -1
bind = $mainMod CTRL SHIFT, period, movecurrentworkspacetomonitor, +1

# Switch workspaces (1-5 on primary, 6-9 on secondary)
bind = $mainMod, 1, workspace, 1
bind = $mainMod, 2, workspace, 2
bind = $mainMod, 3, workspace, 3
bind = $mainMod, 4, workspace, 4
bind = $mainMod, 5, workspace, 5
bind = $mainMod, 6, workspace, 6
bind = $mainMod, 7, workspace, 7
bind = $mainMod, 8, workspace, 8
bind = $mainMod, 9, workspace, 9
bind = $mainMod, 0, workspace, 10

# Move window to workspace
bind = $mainMod SHIFT, 1, movetoworkspace, 1
bind = $mainMod SHIFT, 2, movetoworkspace, 2
bind = $mainMod SHIFT, 3, movetoworkspace, 3
bind = $mainMod SHIFT, 4, movetoworkspace, 4
bind = $mainMod SHIFT, 5, movetoworkspace, 5
bind = $mainMod SHIFT, 6, movetoworkspace, 6
bind = $mainMod SHIFT, 7, movetoworkspace, 7
bind = $mainMod SHIFT, 8, movetoworkspace, 8
bind = $mainMod SHIFT, 9, movetoworkspace, 9
bind = $mainMod SHIFT, 0, movetoworkspace, 10

# Scroll through workspaces
bind = $mainMod, mouse_down, workspace, e+1
bind = $mainMod, mouse_up, workspace, e-1

# Move/resize windows
bindm = $mainMod, mouse:272, movewindow
bindm = $mainMod, mouse:273, resizewindow

# Screenshot bindings
bind = , Print, exec, grim -g "$(slurp)" - | wl-copy
bind = SHIFT, Print, exec, grim - | wl-copy
bind = $mainMod, Print, exec, grim ~/Pictures/screenshot-$(date +%Y%m%d-%H%M%S).png

# Screen lock
bind = $mainMod, L, exec, hyprlock

# Volume control
bind = , XF86AudioRaiseVolume, exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%+
bind = , XF86AudioLowerVolume, exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%-
bind = , XF86AudioMute, exec, wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle

# Media control
bind = , XF86AudioPlay, exec, playerctl play-pause
bind = , XF86AudioNext, exec, playerctl next
bind = , XF86AudioPrev, exec, playerctl previous
```

## Per-Monitor Wallpapers with hyprpaper

Create `~/.config/hypr/hyprpaper.conf`:

```conf
# Preload wallpapers
preload = ~/Pictures/wallpaper-primary.png
preload = ~/Pictures/wallpaper-secondary.png

# Apply wallpapers to specific monitors
wallpaper = DP-1,~/Pictures/wallpaper-primary.png
wallpaper = HDMI-A-1,~/Pictures/wallpaper-secondary.png

# Disable splash
splash = false
```

## Waybar Multi-Monitor Configuration

Waybar automatically spawns on all monitors. Configure per-monitor settings in `~/.config/waybar/config`:

```json
{
    "layer": "top",
    "output": ["DP-1", "HDMI-A-1"],
    
    "modules-left": ["hyprland/workspaces"],
    "modules-center": ["clock"],
    "modules-right": ["pulseaudio", "network", "cpu", "memory", "tray"],
    
    "hyprland/workspaces": {
        "format": "{name}",
        "on-click": "activate",
        "all-outputs": false,
        "persistent-workspaces": {
            "DP-1": [1, 2, 3, 4, 5],
            "HDMI-A-1": [6, 7, 8, 9]
        }
    }
}
```

## Monitor-Specific Tips

### Different Refresh Rates

Hyprland handles mixed refresh rates well. Make sure `vfr` is set appropriately:

```conf
misc {
    vfr = false  # For gaming setups with high refresh
    # vfr = true  # For mixed refresh rates (power saving)
}
```

### VRR (Variable Refresh Rate / G-Sync / FreeSync)

Enable VRR per monitor:

```conf
monitor=DP-1,2560x1440@144,0x0,1,vrr,1
```

### Portrait/Rotated Monitors

```conf
# Rotate monitor 90 degrees clockwise
monitor=HDMI-A-1,1920x1080@60,2560x0,1,transform,1

# Transform values:
# 0 - normal
# 1 - 90 degrees
# 2 - 180 degrees
# 3 - 270 degrees
```

### Disable a Monitor Temporarily

```conf
monitor=HDMI-A-1,disable
```

## Troubleshooting

### Monitor Not Detected

```bash
# Check if monitor is detected
hyprctl monitors

# Force re-scan
hyprctl reload
```

### Wrong Workspace on Monitor

```bash
# Move workspace 6 to HDMI-A-1
hyprctl dispatch moveworkspacetomonitor 6 HDMI-A-1
```

### Window Opens on Wrong Monitor

Use window rules to force specific applications to specific monitors:

```conf
windowrulev2 = monitor DP-1, class:^(gamewindow)$
```
