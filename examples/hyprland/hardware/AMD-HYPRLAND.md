# AMD GPU with Hyprland

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

Comprehensive guide for running Hyprland with AMD GPUs (integrated and discrete) on NixOS.

## AMD Advantages

AMD GPUs generally work better with Wayland than NVIDIA:

✅ Open-source drivers in the kernel  
✅ Better Wayland support  
✅ No proprietary driver issues  
✅ Hardware cursor works out of the box  
✅ Variable refresh rate (FreeSync) support  
✅ Better power management  

## NixOS Configuration

### AMD Integrated Graphics (Ryzen APU)

For AMD Ryzen processors with integrated Radeon graphics:

```nix
{ config, pkgs, ... }:

{
  # Enable Hyprland
  programs.hyprland = {
    enable = true;
    xwayland.enable = true;
  };

  # AMD GPU drivers (already in kernel, just enable OpenGL)
  hardware.opengl = {
    enable = true;
    driSupport = true;
    driSupport32Bit = true;
    
    extraPackages = with pkgs; [
      # RADV (Vulkan)
      mesa.drivers
      
      # AMD GPU compute
      rocmPackages.clr.icd
      
      # Video acceleration
      libvdpau-va-gl
      vaapiVdpau
    ];
  };

  # Load amdgpu kernel module early
  boot.initrd.kernelModules = [ "amdgpu" ];
  
  # Use amdgpu driver
  services.xserver.videoDrivers = [ "amdgpu" ];

  # AMD-specific environment variables
  environment.sessionVariables = {
    # Use RADV Vulkan driver (recommended for gaming)
    AMD_VULKAN_ICD = "RADV";
    
    # Enable GPU acceleration
    LIBVA_DRIVER_NAME = "radeonsi";
    
    # Better performance for some applications
    mesa_glthread = "true";
  };
}
```

### AMD Discrete GPU (RX 6000/7000 series)

For discrete AMD Radeon RX cards:

```nix
{ config, pkgs, ... }:

{
  # Enable Hyprland
  programs.hyprland = {
    enable = true;
    xwayland.enable = true;
  };

  # AMD GPU drivers
  hardware.opengl = {
    enable = true;
    driSupport = true;
    driSupport32Bit = true;
    
    extraPackages = with pkgs; [
      # Vulkan drivers
      amdvlk
      
      # OpenCL
      rocmPackages.clr.icd
      rocmPackages.clr
      rocmPackages.rocm-runtime
      
      # Video acceleration
      libvdpau-va-gl
      vaapiVdpau
    ];
    
    # 32-bit support for gaming
    extraPackages32 = with pkgs; [
      driversi686Linux.amdvlk
    ];
  };

  # Load AMD GPU module
  boot.initrd.kernelModules = [ "amdgpu" ];
  services.xserver.videoDrivers = [ "amdgpu" ];

  # Performance and compatibility
  environment.sessionVariables = {
    AMD_VULKAN_ICD = "RADV";  # or "AMDVLK" for proprietary
    LIBVA_DRIVER_NAME = "radeonsi";
    mesa_glthread = "true";
  };

  # Optional: Enable performance governor
  powerManagement.cpuFreqGovernor = "performance";
}
```

## Hyprland Configuration for AMD

AMD GPUs work great with Hyprland out of the box. Here's an optimised configuration:

```conf
# ~/.config/hypr/hyprland.conf

# Monitor configuration with FreeSync/VRR
# AMD GPUs handle VRR excellently
monitor=DP-1,2560x1440@144,0x0,1,vrr,1

# Environment variables (optional, usually not needed for AMD)
env = AMD_VULKAN_ICD,RADV
env = LIBVA_DRIVER_NAME,radeonsi

# General configuration
general {
    gaps_in = 5
    gaps_out = 10
    border_size = 2
    col.active_border = rgba(33ccffee) rgba(00ff99ee) 45deg
    col.inactive_border = rgba(595959aa)

    layout = dwindle
    allow_tearing = false
}

# Decoration - AMD handles blur very well
decoration {
    rounding = 8

    blur {
        enabled = true
        size = 3
        passes = 2  # AMD can handle more passes
        vibrancy = 0.1696
    }

    drop_shadow = true
    shadow_range = 4
    shadow_render_power = 3
    col.shadow = rgba(1a1a1aee)
}

# Animations - AMD GPUs handle these smoothly
animations {
    enabled = true
    bezier = myBezier, 0.05, 0.9, 0.1, 1.05

    animation = windows, 1, 7, myBezier
    animation = windowsOut, 1, 7, default, popin 80%
    animation = border, 1, 10, default
    animation = borderangle, 1, 8, default
    animation = fade, 1, 7, default
    animation = workspaces, 1, 6, default
}

# Miscellaneous
misc {
    vfr = true  # Variable frame rate works well on AMD
    force_default_wallpaper = 0
    disable_hyprland_logo = true
}

# Gaming optimisations
windowrulev2 = immediate, class:^(steam_app_.*)$
windowrulev2 = immediate, fullscreen:1
```

## Framework Laptop Specific Configuration

For Framework Laptop 13 or 16 with AMD Ryzen:

```nix
{ config, pkgs, ... }:

{
  # Enable Hyprland
  programs.hyprland = {
    enable = true;
    xwayland.enable = true;
  };

  # Framework-specific hardware
  hardware.opengl = {
    enable = true;
    driSupport = true;
    driSupport32Bit = true;
    
    extraPackages = with pkgs; [
      mesa.drivers
      rocmPackages.clr.icd
      libvdpau-va-gl
      vaapiVdpau
    ];
  };

  boot.initrd.kernelModules = [ "amdgpu" ];
  services.xserver.videoDrivers = [ "amdgpu" ];

  # Framework Laptop specific settings
  # XDG portal for screen sharing
  xdg.portal = {
    enable = true;
    extraPortals = with pkgs; [
      xdg-desktop-portal-hyprland
      xdg-desktop-portal-gtk
    ];
  };

  # Brightness control
  programs.light.enable = true;
  
  # Power management (Framework laptops benefit from TLP)
  services.tlp = {
    enable = true;
    settings = {
      CPU_SCALING_GOVERNOR_ON_AC = "performance";
      CPU_SCALING_GOVERNOR_ON_BAT = "powersave";
      
      # Framework-specific: Battery charge thresholds
      START_CHARGE_THRESH_BAT0 = 40;
      STOP_CHARGE_THRESH_BAT0 = 80;
      
      # AMD GPU power management
      RADEON_DPM_STATE_ON_AC = "performance";
      RADEON_DPM_STATE_ON_BAT = "battery";
      RADEON_POWER_PROFILE_ON_AC = "high";
      RADEON_POWER_PROFILE_ON_BAT = "low";
    };
  };

  # Audio
  security.rtkit.enable = true;
  services.pipewire = {
    enable = true;
    alsa.enable = true;
    pulse.enable = true;
  };

  # WiFi and Bluetooth
  networking.networkmanager.enable = true;
  hardware.bluetooth = {
    enable = true;
    powerOnBoot = true;
  };

  # Touchpad
  services.xserver.libinput = {
    enable = true;
    touchpad = {
      tapping = true;
      naturalScrolling = true;
      disableWhileTyping = true;
    };
  };

  # Framework-specific packages
  environment.systemPackages = with pkgs; [
    # Hyprland ecosystem
    hyprpaper
    hyprlock
    hypridle
    waybar
    wofi
    dunst
    grim
    slurp
    wl-clipboard
    
    # Framework utilities
    brightnessctl
    playerctl
    networkmanagerapplet
    blueman
    pavucontrol
    
    # Monitoring
    amdgpu_top  # AMD GPU monitoring tool
  ];
}
```

### Framework Laptop Hyprland Config

```conf
# ~/.config/hypr/hyprland.conf
# Framework Laptop 13/16 with AMD

# Monitor configuration (Framework 13: 2256x1504, Framework 16: 2560x1600)
# Framework 13 (2256x1504 @ 60Hz, 1.175x scaling)
monitor=eDP-1,2256x1504@60,0x0,1.175

# Framework 16 (2560x1600 @ 165Hz, 1.25x scaling)
# monitor=eDP-1,2560x1600@165,0x0,1.25

# Launch applications
exec-once = waybar
exec-once = dunst
exec-once = hypridle
exec-once = hyprpaper
exec-once = nm-applet
exec-once = blueman-applet

# Environment variables
env = XCURSOR_SIZE,24
env = AMD_VULKAN_ICD,RADV
env = LIBVA_DRIVER_NAME,radeonsi

# Input configuration
input {
    kb_layout = gb
    kb_variant =
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

# General
general {
    gaps_in = 4
    gaps_out = 8
    border_size = 2
    col.active_border = rgba(33ccffee) rgba(00ff99ee) 45deg
    col.inactive_border = rgba(595959aa)

    layout = dwindle
}

# Decoration (AMD handles blur well, but disable on battery for efficiency)
decoration {
    rounding = 8

    blur {
        enabled = true  # Set to false for better battery life
        size = 3
        passes = 1
    }

    drop_shadow = true
    shadow_range = 4
    shadow_render_power = 3
    col.shadow = rgba(1a1a1aee)
}

# Animations (balanced for battery life)
animations {
    enabled = true
    bezier = easeOut, 0.16, 1, 0.3, 1

    animation = windows, 1, 4, easeOut
    animation = windowsOut, 1, 4, default, popin 80%
    animation = border, 1, 5, default
    animation = fade, 1, 4, default
    animation = workspaces, 1, 4, easeOut, slide
}

# Gestures (essential for Framework trackpad)
gestures {
    workspace_swipe = true
    workspace_swipe_fingers = 3
    workspace_swipe_distance = 300
}

# Miscellaneous
misc {
    vfr = true
    force_default_wallpaper = 0
    disable_hyprland_logo = true
    mouse_move_enables_dpms = true
    key_press_enables_dpms = true
}

# Laptop function keys
bind = , XF86AudioRaiseVolume, exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%+
bind = , XF86AudioLowerVolume, exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%-
bind = , XF86AudioMute, exec, wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle
bind = , XF86AudioMicMute, exec, wpctl set-mute @DEFAULT_AUDIO_SOURCE@ toggle

bind = , XF86MonBrightnessUp, exec, brightnessctl set +10%
bind = , XF86MonBrightnessDown, exec, brightnessctl set 10%-

bind = , XF86AudioPlay, exec, playerctl play-pause
bind = , XF86AudioNext, exec, playerctl next
bind = , XF86AudioPrev, exec, playerctl previous

# Screen lock
bind = SUPER, L, exec, hyprlock
```

## Performance Tuning

### Check GPU Usage

```bash
# AMD GPU monitoring
amdgpu_top

# Or radeontop
radeontop

# General GPU info
lspci -k | grep -A 3 -i vga
```

Install monitoring tools:

```nix
environment.systemPackages = with pkgs; [
  amdgpu_top
  radeontop
];
```

### Force Performance Mode

For gaming or heavy workloads:

```bash
# Check current power state
cat /sys/class/drm/card0/device/power_dpm_state

# Set to performance (temporary)
echo "performance" | sudo tee /sys/class/drm/card0/device/power_dpm_force_performance_level
```

Make permanent in NixOS:

```nix
systemd.tmpfiles.rules = [
  "w /sys/class/drm/card0/device/power_dpm_force_performance_level - - - - performance"
];
```

### Vulkan Driver Selection

AMD provides two Vulkan drivers:

1. **RADV** (Mesa, open-source) - Recommended for most users
2. **AMDVLK** (AMD official) - Better for some professional applications

```nix
# Use RADV (default, recommended)
environment.sessionVariables = {
  AMD_VULKAN_ICD = "RADV";
};

# Or use AMDVLK
environment.sessionVariables = {
  AMD_VULKAN_ICD = "AMDVLK";
};
```

## Troubleshooting

### Screen Tearing

AMD GPUs rarely have tearing issues with Wayland, but if you do:

```conf
# Enable VRR
monitor=DP-1,2560x1440@144,0x0,1,vrr,1

# Or adjust VFR
misc {
    vfr = false
}
```

### Poor Performance

**Check if using correct driver:**

```bash
glxinfo | grep "OpenGL renderer"
# Should show: AMD Radeon...

vulkaninfo | grep "deviceName"
# Should show: AMD RADV...
```

**Enable performance governor:**

```nix
powerManagement.cpuFreqGovernor = "performance";
```

### Video Playback Issues

**Enable hardware acceleration:**

```nix
hardware.opengl.extraPackages = with pkgs; [
  libvdpau-va-gl
  vaapiVdpau
];

environment.sessionVariables = {
  LIBVA_DRIVER_NAME = "radeonsi";
};
```

**Test hardware acceleration:**

```bash
# Install mpv
nix-shell -p mpv

# Test hardware decoding
mpv --hwdec=auto video.mp4
```

### Gaming Performance

**For Steam games:**

```nix
programs.steam = {
  enable = true;
  remotePlay.openFirewall = true;
  dedicatedServer.openFirewall = true;
};

# Enable GameMode
programs.gamemode.enable = true;
```

**Launch games with:**

```bash
# Enable AMD performance features
gamemoderun %command%
```

**Hyprland gaming window rules:**

```conf
# Disable compositor overhead for games
windowrulev2 = immediate, class:^(steam_app_.*)$
windowrulev2 = immediate, fullscreen:1

# Force specific games to fullscreen
windowrulev2 = fullscreen, class:^(steam_app_570)$ # Dota 2
```

## Multi-Monitor with AMD

AMD handles multi-monitor setups excellently:

```conf
# Primary monitor (high refresh rate)
monitor=DP-1,2560x1440@144,0x0,1,vrr,1

# Secondary monitor
monitor=HDMI-A-1,1920x1080@60,2560x0,1,vrr,1

# Mixed refresh rates work well on AMD
misc {
    vfr = true  # AMD handles this smoothly
}
```

## Power Management (Laptops)

For Framework and other AMD laptops:

```nix
# TLP for battery optimisation
services.tlp = {
  enable = true;
  settings = {
    CPU_SCALING_GOVERNOR_ON_AC = "performance";
    CPU_SCALING_GOVERNOR_ON_BAT = "powersave";
    
    # AMD-specific
    RADEON_DPM_STATE_ON_AC = "performance";
    RADEON_DPM_STATE_ON_BAT = "battery";
    RADEON_POWER_PROFILE_ON_AC = "high";
    RADEON_POWER_PROFILE_ON_BAT = "low";
  };
};
```

## Testing Your Setup

```bash
# Check Wayland session
echo $XDG_SESSION_TYPE  # Should output: wayland

# Check AMD driver
lspci -k | grep -A 3 -i vga

# Check Vulkan
vulkaninfo | grep -i amd

# Check OpenGL
glxinfo | grep -i amd

# Monitor GPU usage during Hyprland
amdgpu_top
```

## Known Working Configurations

### Framework Laptop 13 (AMD Ryzen 7 7840U)

✅ 2256x1504 @ 60Hz with 1.175x scaling  
✅ Full hardware acceleration  
✅ Excellent battery life with VFR  
✅ FreeSync works perfectly  

### Framework Laptop 16 (AMD Ryzen 9 7940HS + Radeon RX 7700S)

✅ 2560x1600 @ 165Hz with 1.25x scaling  
✅ Hybrid graphics (iGPU + dGPU)  
✅ Excellent gaming performance  
✅ VRR at 165Hz works flawlessly  

### Desktop (Ryzen 9 7950X + RX 7900 XTX)

✅ 4K @ 144Hz with VRR  
✅ Triple monitor setups work perfectly  
✅ Zero screen tearing  
✅ Excellent gaming performance with immediate window rule  

## Comparison: AMD vs NVIDIA for Hyprland

| Feature              | AMD              | NVIDIA                    |
| -------------------- | ---------------- | ------------------------- |
| Wayland support      | ✅ Excellent     | ⚠️ Requires workarounds   |
| Hardware cursor      | ✅ Works         | ❌ Needs WLR workaround   |
| Screen tearing       | ✅ Rare          | ⚠️ Common                 |
| Driver setup         | ✅ Automatic     | ⚠️ Manual configuration   |
| Power management     | ✅ Excellent     | ⚠️ Inconsistent           |
| VRR/FreeSync support | ✅ Perfect       | ⚠️ Requires env vars      |
| Multi-monitor        | ✅ Seamless      | ⚠️ Can be problematic     |
| Gaming performance   | ✅ Great (RADV)  | ✅ Great (when it works)  |

**Recommendation:** AMD is the better choice for Hyprland on NixOS.
