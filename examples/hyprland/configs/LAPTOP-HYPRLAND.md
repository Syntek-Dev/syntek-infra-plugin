# Laptop-Specific Hyprland Configuration

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

Optimised configuration for laptops with focus on battery life and touchpad gestures.

## Key Features

- Power-efficient animations and effects
- HiDPI scaling support
- Laptop function key bindings
- Touchpad gestures
- Battery-friendly compositor settings

## hyprland.conf

```conf
# Hyprland Laptop Configuration
# Optimised for battery life and mobile use

# Monitor configuration (HiDPI example)
# Adjust for your laptop display
monitor=eDP-1,2880x1800@90,0x0,1.5  # 1.5x scaling for HiDPI

# Execute at launch
exec-once = waybar
exec-once = dunst
exec-once = hypridle  # Idle management for laptops
exec-once = hyprpaper
exec-once = nm-applet  # NetworkManager applet
exec-once = blueman-applet  # Bluetooth applet

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
        drag_lock = true
        middle_button_emulation = true
    }

    sensitivity = 0
}

# General window configuration
general {
    gaps_in = 4
    gaps_out = 8
    border_size = 2
    col.active_border = rgba(33ccffee) rgba(00ff99ee) 45deg
    col.inactive_border = rgba(595959aa)

    layout = dwindle

    allow_tearing = false
}

# Decoration - optimised for battery life
decoration {
    rounding = 6

    blur {
        enabled = false  # Disabled for battery savings
    }

    drop_shadow = false  # Disabled for battery savings
}

# Animations - lightweight for battery
animations {
    enabled = true

    # Gentle bezier curve
    bezier = easeOut, 0.16, 1, 0.3, 1

    # Faster, simpler animations
    animation = windows, 1, 3, easeOut
    animation = windowsOut, 1, 3, default, popin 80%
    animation = border, 1, 4, default
    animation = fade, 1, 3, default
    animation = workspaces, 1, 3, easeOut, slide
}

# Layouts
dwindle {
    pseudotile = true
    preserve_split = true
    no_gaps_when_only = false
}

master {
    new_is_master = true
}

# Gestures - essential for laptops
gestures {
    workspace_swipe = true
    workspace_swipe_fingers = 3
    workspace_swipe_distance = 300
    workspace_swipe_cancel_ratio = 0.5
}

# Miscellaneous - power saving
misc {
    force_default_wallpaper = 0
    disable_hyprland_logo = true
    vfr = true  # Variable frame rate saves battery
    mouse_move_enables_dpms = true
    key_press_enables_dpms = true
}

# Window rules
windowrulev2 = float, class:^(pavucontrol)$
windowrulev2 = float, class:^(nm-connection-editor)$
windowrulev2 = float, class:^(blueman-manager)$
windowrulev2 = float, class:^(org.gnome.Calculator)$

# Application-specific battery optimisation
windowrulev2 = immediate, class:^(firefox)$  # Better scrolling performance

# Keybindings
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

# Switch workspaces
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

# Move active window to workspace
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

# Screen lock (essential for laptops)
bind = $mainMod, L, exec, hyprlock
bind = $mainMod SHIFT, L, exec, systemctl suspend

# LAPTOP FUNCTION KEYS

# Volume control
bind = , XF86AudioRaiseVolume, exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%+
bind = , XF86AudioLowerVolume, exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%-
bind = , XF86AudioMute, exec, wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle
bind = , XF86AudioMicMute, exec, wpctl set-mute @DEFAULT_AUDIO_SOURCE@ toggle

# Brightness control
bind = , XF86MonBrightnessUp, exec, light -A 10
bind = , XF86MonBrightnessDown, exec, light -S 10

# Media control
bind = , XF86AudioPlay, exec, playerctl play-pause
bind = , XF86AudioPause, exec, playerctl play-pause
bind = , XF86AudioNext, exec, playerctl next
bind = , XF86AudioPrev, exec, playerctl previous

# Display toggle (for presentations)
bind = , XF86Display, exec, ~/scripts/toggle-display.sh

# Airplane mode toggle (if supported)
bind = , XF86WLAN, exec, nmcli radio wifi toggle

# Keyboard backlight (if supported)
bind = , XF86KbdBrightnessUp, exec, brightnessctl --device='*::kbd_backlight' set +10%
bind = , XF86KbdBrightnessDown, exec, brightnessctl --device='*::kbd_backlight' set 10%-
```

## hypridle.conf

Idle management for automatic screen lock and suspend:

```conf
# ~/.config/hypr/hypridle.conf

general {
    lock_cmd = pidof hyprlock || hyprlock       # Lock on timeout
    before_sleep_cmd = loginctl lock-session    # Lock before suspend
    after_sleep_cmd = hyprctl dispatch dpms on  # Wake displays after suspend
}

listener {
    timeout = 300                                 # 5 minutes
    on-timeout = light -O && light -S 10          # Save and dim brightness
    on-resume = light -I                          # Restore brightness
}

listener {
    timeout = 600                                 # 10 minutes
    on-timeout = loginctl lock-session            # Lock screen
}

listener {
    timeout = 900                                 # 15 minutes
    on-timeout = hyprctl dispatch dpms off        # Turn off displays
    on-resume = hyprctl dispatch dpms on          # Turn on displays
}

listener {
    timeout = 1800                                # 30 minutes
    on-timeout = systemctl suspend                # Suspend system
}
```

## Battery Status in Waybar

Add battery module to waybar config:

```json
"battery": {
    "states": {
        "warning": 30,
        "critical": 15
    },
    "format": "{icon} {capacity}%",
    "format-charging": " {capacity}%",
    "format-plugged": " {capacity}%",
    "format-icons": ["", "", "", "", ""]
}
```

## Power-Saving Tips

### Disable Blur and Shadows

Already done in this config for battery savings.

### Use VFR (Variable Frame Rate)

```conf
misc {
    vfr = true
}
```

### Reduce Animation Speed

```conf
animations {
    animation = windows, 1, 2, easeOut  # Even faster
}
```

### Monitor Power Management

Automatically turn off display and suspend:

```bash
# Install hypridle
# Configuration in ~/.config/hypr/hypridle.conf (shown above)
```

## HiDPI Scaling

For high-resolution laptop displays:

```conf
# 4K display at 2x scaling
monitor=eDP-1,3840x2160@60,0x0,2

# 2880x1800 at 1.5x scaling (MacBook-like)
monitor=eDP-1,2880x1800@90,0x0,1.5

# 2560x1440 at 1.25x scaling
monitor=eDP-1,2560x1440@144,0x0,1.25
```

## Required NixOS Configuration

```nix
# Enable brightness control
programs.light.enable = true;
users.users.YOUR_USER.extraGroups = [ "video" ];

# Audio support
security.rtkit.enable = true;
services.pipewire = {
  enable = true;
  alsa.enable = true;
  pulse.enable = true;
};

# Bluetooth
hardware.bluetooth = {
  enable = true;
  powerOnBoot = true;
};
services.blueman.enable = true;

# NetworkManager
networking.networkmanager.enable = true;
```
