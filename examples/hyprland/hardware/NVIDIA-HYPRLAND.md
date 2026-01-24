# NVIDIA GPU with Hyprland

Comprehensive guide for running Hyprland with NVIDIA GPUs on NixOS.

## Known Issues

NVIDIA + Wayland has historically been problematic. Recent driver versions (525+) have improved significantly, but some issues remain:

- Hardware cursor issues (workaround available)
- Screen tearing in some applications
- Performance may vary compared to X11

## NixOS Configuration

```nix
{ config, pkgs, lib, ... }:

{
  # Enable Hyprland
  programs.hyprland = {
    enable = true;
    xwayland.enable = true;
  };

  # NVIDIA drivers
  services.xserver.videoDrivers = [ "nvidia" ];
  
  hardware.nvidia = {
    # Enable modesetting (required for Wayland)
    modesetting.enable = true;
    
    # Enable power management (helps with suspend/resume)
    powerManagement.enable = true;
    
    # Use open source kernel modules (optional, experimental)
    open = false;  # Set to true for open-source drivers (RTX 20 series+)
    
    # Enable nvidia-settings
    nvidiaSettings = true;
    
    # Select driver version
    package = config.boot.kernelPackages.nvidiaPackages.stable;
    # package = config.boot.kernelPackages.nvidiaPackages.beta;
    # package = config.boot.kernelPackages.nvidiaPackages.production;
  };

  # OpenGL support
  hardware.opengl = {
    enable = true;
    driSupport = true;
    driSupport32Bit = true;
  };

  # Critical environment variables for NVIDIA + Hyprland
  environment.sessionVariables = {
    # NVIDIA driver library
    LIBVA_DRIVER_NAME = "nvidia";
    
    # Force Wayland backend
    XDG_SESSION_TYPE = "wayland";
    GBM_BACKEND = "nvidia-drm";
    __GLX_VENDOR_LIBRARY_NAME = "nvidia";
    
    # Fix cursor issues
    WLR_NO_HARDWARE_CURSORS = "1";
    
    # Uncomment if experiencing flickering
    # __GL_GSYNC_ALLOWED = "0";
    # __GL_VRR_ALLOWED = "0";
  };

  # Kernel parameters for NVIDIA
  boot.kernelParams = [ 
    "nvidia-drm.modeset=1"
    "nvidia.NVreg_PreserveVideoMemoryAllocations=1"  # For suspend/resume
  ];

  # Load NVIDIA modules early
  boot.initrd.kernelModules = [ "nvidia" "nvidia_modeset" "nvidia_uvm" "nvidia_drm" ];
}
```

## Hyprland Configuration for NVIDIA

Add to your `~/.config/hypr/hyprland.conf`:

```conf
# NVIDIA-specific environment variables
env = LIBVA_DRIVER_NAME,nvidia
env = XDG_SESSION_TYPE,wayland
env = GBM_BACKEND,nvidia-drm
env = __GLX_VENDOR_LIBRARY_NAME,nvidia
env = WLR_NO_HARDWARE_CURSORS,1

# Monitor configuration with VRR (G-Sync/FreeSync)
# Enable VRR per monitor: vrr,1 or vrr,2
monitor=DP-1,2560x1440@144,0x0,1,vrr,1

# Disable VFR for smoother experience with NVIDIA
misc {
    vfr = false
}

# Gaming optimisations
windowrulev2 = immediate, class:^(cs2)$
windowrulev2 = immediate, class:^(steam_app_.*)$
windowrulev2 = immediate, fullscreen:1

# Reduce blur and shadows if experiencing performance issues
decoration {
    blur {
        enabled = false  # Disable for better performance
    }
    drop_shadow = false  # Disable for better performance
}
```

## Troubleshooting

### Cursor Disappears or Flickers

**Solution:** Set `WLR_NO_HARDWARE_CURSORS=1`

Already included in the NixOS configuration above.

### Screen Tearing

**Solution 1:** Enable VRR/G-Sync

```conf
monitor=DP-1,2560x1440@144,0x0,1,vrr,1
```

**Solution 2:** Disable triple buffering

```nix
environment.sessionVariables = {
  __GL_YIELD = "USLEEP";
};
```

**Solution 3:** Force composition pipeline (may reduce performance)

```nix
hardware.nvidia = {
  forceFullCompositionPipeline = true;
};
```

### Black Screen on Login

**Check driver version:**

```bash
nvidia-smi
```

Ensure you're on driver version 525 or newer. Update if necessary:

```nix
hardware.nvidia.package = config.boot.kernelPackages.nvidiaPackages.beta;
```

**Check kernel parameters:**

```bash
cat /proc/cmdline | grep nvidia
```

Should include `nvidia-drm.modeset=1`.

### Poor Performance

**1. Disable blur and shadows** (shown above in Hyprland config)

**2. Check if using dedicated GPU:**

```bash
lspci -k | grep -A 2 -E "(VGA|3D)"
```

**3. Force performance mode:**

```bash
nvidia-settings -a "[gpu:0]/GpuPowerMizerMode=1"
```

Make persistent:

```nix
environment.etc."X11/xorg.conf.d/20-nvidia.conf".text = ''
  Section "Device"
    Identifier "NVIDIA Card"
    Driver     "nvidia"
    Option     "RegistryDwords" "PowerMizerEnable=0x1; PerfLevelSrc=0x2222; PowerMizerDefault=0x3; PowerMizerDefaultAC=0x3"
  EndSection
'';
```

### Suspend/Resume Issues

**Enable power management:**

```nix
hardware.nvidia.powerManagement.enable = true;

boot.kernelParams = [
  "nvidia.NVreg_PreserveVideoMemoryAllocations=1"
];
```

**Create systemd service for resume:**

```nix
systemd.services.nvidia-resume = {
  description = "NVIDIA Resume";
  after = [ "suspend.target" "hibernate.target" "hybrid-sleep.target" ];
  wantedBy = [ "suspend.target" "hibernate.target" "hybrid-sleep.target" ];
  
  serviceConfig = {
    Type = "oneshot";
    ExecStart = "${pkgs.kmod}/bin/modprobe nvidia";
  };
};
```

### Application-Specific Issues

**Firefox flickering:**

Add to `about:config`:
- `gfx.webrender.all` → `true`
- `widget.wayland.dmabuf-vaapi.enabled` → `true`

**Chromium/Chrome:**

Launch with flags:
```bash
chromium --enable-features=UseOzonePlatform --ozone-platform=wayland
```

Or add to `.bashrc`:
```bash
export CHROME_EXECUTABLE="chromium"
export CHROMIUM_FLAGS="--enable-features=UseOzonePlatform --ozone-platform=wayland"
```

**OBS Studio:**

Use Wayland capture instead of X11:
- Source → Screen Capture (PipeWire)

**Steam games:**

Add to game launch options:
```
SDL_VIDEODRIVER=wayland %command%
```

Or force X11 for problematic games:
```
SDL_VIDEODRIVER=x11 %command%
```

## Multi-Monitor with NVIDIA

```conf
# Primary monitor (high refresh rate)
monitor=DP-1,2560x1440@144,0x0,1,vrr,1

# Secondary monitor
monitor=HDMI-A-1,1920x1080@60,2560x0,1

# Workspace binding
workspace=1,monitor:DP-1,default:true
workspace=2,monitor:DP-1
workspace=3,monitor:DP-1
workspace=6,monitor:HDMI-A-1,default:true
workspace=7,monitor:HDMI-A-1
```

## Laptop with NVIDIA Optimus

For laptops with both Intel/AMD and NVIDIA GPUs:

```nix
{ config, pkgs, ... }:

{
  # NVIDIA drivers
  services.xserver.videoDrivers = [ "nvidia" ];
  
  hardware.nvidia = {
    modesetting.enable = true;
    powerManagement.enable = true;
    
    # Enable PRIME
    prime = {
      # Find bus IDs with: lspci | grep -E "VGA|3D"
      intelBusId = "PCI:0:2:0";
      nvidiaBusId = "PCI:1:0:0";
      
      # Choose one mode:
      
      # Option 1: Offload mode (battery saving, use NVIDIA on demand)
      offload.enable = true;
      offload.enableOffloadCmd = true;  # Provides nvidia-offload command
      
      # Option 2: Sync mode (always use NVIDIA, better performance)
      # sync.enable = true;
      
      # Option 3: Reverse sync (always use Intel, NVIDIA for rendering)
      # reverseSync.enable = true;
    };
    
    package = config.boot.kernelPackages.nvidiaPackages.stable;
  };

  # Intel GPU support
  hardware.opengl = {
    enable = true;
    extraPackages = with pkgs; [
      intel-media-driver
      vaapiIntel
    ];
  };
}
```

**Usage with offload mode:**

```bash
# Run application with NVIDIA GPU
nvidia-offload hyprland

# Or for specific applications
nvidia-offload steam
```

## Performance Tuning

### Force Maximum Performance

```bash
# Check current power mode
nvidia-smi -q -d PERFORMANCE

# Set to maximum performance
nvidia-settings -a "[gpu:0]/GpuPowerMizerMode=1"
```

### Monitor GPU Usage

```bash
# Real-time monitoring
nvidia-smi -l 1

# Or use nvtop
nvtop
```

```nix
environment.systemPackages = with pkgs; [
  nvtop  # NVIDIA GPU monitor
];
```

## Recommended Driver Versions

| GPU Series | Minimum Driver | Recommended |
|------------|----------------|-------------|
| RTX 40 series | 525.60 | Latest stable |
| RTX 30 series | 525.60 | Latest stable |
| RTX 20 series | 525.60 | Latest stable |
| GTX 16 series | 525.60 | Latest stable |
| GTX 10 series | 525.60 | 535.xx+ |

## Testing Your Setup

```bash
# Check Wayland session
echo $XDG_SESSION_TYPE  # Should output: wayland

# Check NVIDIA driver
nvidia-smi

# Check Hyprland is using NVIDIA
hyprctl version

# Test GPU rendering
glxinfo | grep "OpenGL renderer"

# Monitor GPU during Hyprland session
nvidia-smi -l 1
```

## Known Working Configurations

### RTX 3080 + 2560x1440@144Hz

```nix
hardware.nvidia = {
  modesetting.enable = true;
  package = config.boot.kernelPackages.nvidiaPackages.stable;
};
```

```conf
monitor=DP-1,2560x1440@144,0x0,1,vrr,1
misc { vfr = false }
```

### GTX 1080 + 1920x1080@60Hz

```nix
hardware.nvidia = {
  modesetting.enable = true;
  package = config.boot.kernelPackages.nvidiaPackages.production;
};
```

```conf
monitor=HDMI-A-1,1920x1080@60,0x0,1
```

## Alternative: Use X11 Instead

If Wayland issues are insurmountable, consider using Hyprland's X11 fallback or alternative window managers:

- **i3** - Tiling window manager for X11
- **bspwm** - Binary space partitioning window manager for X11
- **Sway** - i3-compatible Wayland compositor (may have similar NVIDIA issues)

However, Hyprland is Wayland-only, so this would mean switching compositors entirely.
