# Basic Hyprland Configuration

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

A minimal, well-commented Hyprland configuration suitable for laptops and desktops.

## File Structure

```
~/.config/hypr/
├── hyprland.conf       # Main configuration (this file)
├── monitors.conf       # Monitor settings (optional, can be inline)
├── keybinds.conf       # Keybindings (optional, can be inline)
└── autostart.conf      # Startup applications (optional)
```

## hyprland.conf

```conf
# Hyprland Configuration
# See https://wiki.hyprland.org/ for documentation

# Monitor configuration
# Format: monitor=name,resolution@refresh,position,scale
monitor=,preferred,auto,1

# Source additional config files (optional)
# source = ~/.config/hypr/monitors.conf
# source = ~/.config/hypr/keybinds.conf
# source = ~/.config/hypr/autostart.conf

# Execute at launch
exec-once = waybar
exec-once = dunst
exec-once = hyprpaper

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

    touchpad {
        natural_scroll = true
        disable_while_typing = true
        tap-to-click = true
    }

    sensitivity = 0  # -1.0 - 1.0, 0 means no modification
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

# Decoration (shadows, blur, etc.)
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

    # Bezier curves for animations
    bezier = myBezier, 0.05, 0.9, 0.1, 1.05

    # Animation definitions
    # Format: animation=NAME,ONOFF,SPEED,CURVE[,STYLE]
    animation = windows, 1, 7, myBezier
    animation = windowsOut, 1, 7, default, popin 80%
    animation = border, 1, 10, default
    animation = borderangle, 1, 8, default
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

# Gestures (laptop touchpad)
gestures {
    workspace_swipe = true
    workspace_swipe_fingers = 3
}

# Miscellaneous
misc {
    force_default_wallpaper = 0
    disable_hyprland_logo = true
    vfr = true  # Variable frame rate (saves power)
}

# Window rules
# Example: windowrulev2 = float,class:^(kitty)$,title:^(kitty)$
windowrulev2 = float, class:^(pavucontrol)$
windowrulev2 = float, class:^(nm-connection-editor)$
windowrulev2 = float, class:^(blueman-manager)$

# Keybindings
$mainMod = SUPER

# Application bindings
bind = $mainMod, Q, exec, kitty
bind = $mainMod, C, killactive,
bind = $mainMod, M, exit,
bind = $mainMod, E, exec, thunar
bind = $mainMod, V, togglefloating,
bind = $mainMod, R, exec, wofi --show drun
bind = $mainMod, P, pseudo, # dwindle
bind = $mainMod, J, togglesplit, # dwindle

# Move focus with mainMod + vim keys
bind = $mainMod, h, movefocus, l
bind = $mainMod, l, movefocus, r
bind = $mainMod, k, movefocus, u
bind = $mainMod, j, movefocus, d

# Move focus with mainMod + arrow keys
bind = $mainMod, left, movefocus, l
bind = $mainMod, right, movefocus, r
bind = $mainMod, up, movefocus, u
bind = $mainMod, down, movefocus, d

# Switch workspaces with mainMod + [0-9]
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

# Move active window to workspace with mainMod + SHIFT + [0-9]
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

# Scroll through existing workspaces with mainMod + scroll
bind = $mainMod, mouse_down, workspace, e+1
bind = $mainMod, mouse_up, workspace, e-1

# Move/resize windows with mainMod + LMB/RMB and dragging
bindm = $mainMod, mouse:272, movewindow
bindm = $mainMod, mouse:273, resizewindow

# Screenshot bindings
bind = , Print, exec, grim -g "$(slurp)" - | wl-copy
bind = SHIFT, Print, exec, grim - | wl-copy

# Screen lock
bind = $mainMod, L, exec, hyprlock

# Volume control (laptop function keys)
bind = , XF86AudioRaiseVolume, exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%+
bind = , XF86AudioLowerVolume, exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%-
bind = , XF86AudioMute, exec, wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle

# Brightness control (laptop function keys)
bind = , XF86MonBrightnessUp, exec, light -A 10
bind = , XF86MonBrightnessDown, exec, light -S 10
```

## Usage

1. Copy this configuration to `~/.config/hypr/hyprland.conf`
2. Adjust monitor configuration for your setup (use `hyprctl monitors` to see available monitors)
3. Modify keybindings to your preference
4. Reload with `hyprctl reload` or restart Hyprland

## Required Packages

Make sure these are installed (via NixOS configuration or other package manager):

- `hyprland` - The compositor
- `waybar` - Status bar
- `dunst` - Notification daemon
- `hyprpaper` - Wallpaper manager
- `kitty` - Terminal emulator
- `wofi` - Application launcher
- `grim` + `slurp` - Screenshots
- `wl-clipboard` - Clipboard utilities
- `hyprlock` - Screen locker
- `light` - Brightness control
- `pipewire` + `wireplumber` - Audio (wpctl command)

## Customisation

### Change Border Colours

```conf
col.active_border = rgba(ff6600ee)    # Orange active border
col.inactive_border = rgba(444444aa)  # Grey inactive border
```

### Disable Blur (for performance)

```conf
decoration {
    blur {
        enabled = false
    }
}
```

### Change Animation Speed

```conf
animations {
    # Faster animations (lower number = faster)
    animation = windows, 1, 4, myBezier
    animation = workspaces, 1, 3, default
}
```
