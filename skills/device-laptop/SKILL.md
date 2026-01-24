# Device: Developer Laptop

This skill provides guidance for configuring NixOS on developer laptops.

## Hardware Considerations

### Power Management

```nix
# TLP for battery optimisation
services.tlp = {
  enable = true;
  settings = {
    CPU_SCALING_GOVERNOR_ON_AC = "performance";
    CPU_SCALING_GOVERNOR_ON_BAT = "powersave";
    START_CHARGE_THRESH_BAT0 = 40;
    STOP_CHARGE_THRESH_BAT0 = 80;
  };
};

# Power profiles daemon (alternative to TLP)
services.power-profiles-daemon.enable = true;
```

### Display & Graphics

```nix
# Intel GPU
hardware.opengl = {
  enable = true;
  extraPackages = with pkgs; [
    intel-media-driver
    vaapiIntel
    vaapiVdpau
    libvdpau-va-gl
  ];
};

# AMD GPU (integrated or discrete)
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

# HiDPI scaling
services.xserver.dpi = 192;  # Adjust for your display

# NVIDIA (if applicable)
services.xserver.videoDrivers = [ "nvidia" ];
hardware.nvidia = {
  modesetting.enable = true;
  powerManagement.enable = true;
};
```

### Wireless Networking

```nix
# WiFi
networking.wireless.enable = false;  # Use NetworkManager instead
networking.networkmanager.enable = true;

# Bluetooth
hardware.bluetooth = {
  enable = true;
  powerOnBoot = true;
};
services.blueman.enable = true;
```

### Touchpad

```nix
services.xserver.libinput = {
  enable = true;
  touchpad = {
    tapping = true;
    naturalScrolling = true;
    disableWhileTyping = true;
  };
};
```

## Development Environment

### Editors

```nix
# Zed editor (primary)
environment.systemPackages = with pkgs; [
  zed-editor
];

# Neovim (planned primary)
programs.neovim = {
  enable = true;
  defaultEditor = true;
  viAlias = true;
  vimAlias = true;
};

# VS Code (backup)
environment.systemPackages = with pkgs; [
  vscode
];
```

### Development Tools

```nix
environment.systemPackages = with pkgs; [
  # Version control
  git
  gh  # GitHub CLI

  # Build tools
  gnumake
  cmake

  # Languages
  rustup
  nodejs_20
  python3

  # Containers
  docker-compose

  # Utilities
  ripgrep
  fd
  jq
  yq
  httpie

  # Terminal
  kitty
  tmux
  starship
];
```

### Docker

```nix
virtualisation.docker = {
  enable = true;
  enableOnBoot = true;
  # Use rootless mode for security
  rootless = {
    enable = true;
    setSocketVariable = true;
  };
};

# Add user to docker group (if not using rootless)
users.users.USERNAME.extraGroups = [ "docker" ];
```

## Wireguard Configuration

### Outbound VPN (Mullvad)

Primary use case for laptops - privacy protection:

```nix
networking.wireguard.interfaces.mullvad = {
  ips = [ "10.66.x.x/32" ];
  privateKeyFile = "/run/secrets/mullvad-private-key";

  peers = [{
    publicKey = "MULLVAD_SERVER_PUBLIC_KEY";
    allowedIPs = [ "0.0.0.0/0" "::/0" ];
    endpoint = "SERVER:51820";
  }];
};
```

### Home Access VPN (Optional)

For accessing home network when travelling:

```nix
networking.wireguard.interfaces.home = {
  ips = [ "10.100.0.2/24" ];
  privateKeyFile = "/run/secrets/home-wireguard-key";

  peers = [{
    publicKey = "HOME_SERVER_PUBLIC_KEY";
    allowedIPs = [ "10.100.0.0/24" "192.168.1.0/24" ];
    endpoint = "home.example.com:51820";
    persistentKeepalive = 25;
  }];
};
```

## Profile Options

### Minimal Profile

Basic laptop setup without development tools:

- Boot loader and kernel
- Networking (WiFi, Bluetooth)
- Power management
- Basic desktop environment
- Terminal and shell

### Full Profile (Batteries-Included)

Complete developer workstation:

- Everything in minimal
- Zed, Neovim, VS Code
- Docker and containers
- All language toolchains (Rust, Node, Python)
- Development utilities
- Wireguard VPN (Mullvad)

## Zed Config Integration

The Zed editor configuration can be integrated from:
`/home/sam-dev/Repos/personal/zedconfig/`

Key files:

- `install.sh` - Main installer
- `config/zed/` - Zed configuration files
- `config/git/` - Git configuration

## Security

### Firewall

```nix
networking.firewall = {
  enable = true;
  allowedTCPPorts = [ ];  # No incoming by default
  allowedUDPPorts = [ 51820 ];  # Wireguard
};
```

### Disk Encryption

```nix
# LUKS encryption (set up during installation)
boot.initrd.luks.devices."cryptroot" = {
  device = "/dev/disk/by-uuid/YOUR-UUID";
  preLVM = true;
};
```

## Hyprland Configuration

### Basic Hyprland Setup

```nix
programs.hyprland = {
  enable = true;
  xwayland.enable = true;
};

# Essential Hyprland ecosystem
environment.systemPackages = with pkgs; [
  waybar           # Status bar
  wofi             # Application launcher
  dunst            # Notifications
  grim             # Screenshots
  slurp            # Screen area selection
  wl-clipboard     # Clipboard utilities
  hyprlock         # Screen locker
  hypridle         # Idle management
  hyprpaper        # Wallpaper manager
];
```

### Laptop-Specific Hyprland Features

```nix
# Enable XDG desktop portal for screen sharing
xdg.portal = {
  enable = true;
  extraPortals = with pkgs; [
    xdg-desktop-portal-hyprland
    xdg-desktop-portal-gtk
  ];
};

# Brightness control
programs.light.enable = true;
users.users.USERNAME.extraGroups = [ "video" ];

# Audio control
security.rtkit.enable = true;
services.pipewire = {
  enable = true;
  alsa.enable = true;
  pulse.enable = true;
};
```

### Custom hyprland.conf Location

User configuration typically goes in:

- `~/.config/hypr/hyprland.conf` - Main config
- `~/.config/hypr/monitors.conf` - Monitor settings
- `~/.config/hypr/keybinds.conf` - Keybindings
- `~/.config/hypr/windowrules.conf` - Window rules
- `~/.config/hypr/animations.conf` - Animations
- `~/.config/hypr/autostart.conf` - Startup apps

### Laptop HiDPI Configuration

For high-resolution laptop displays:

```conf
# MacBook-like displays (2880x1800)
monitor=eDP-1,2880x1800@90,0x0,1.5

# Framework Laptop 13 (2256x1504)
monitor=eDP-1,2256x1504@60,0x0,1.175

# Framework Laptop 16 (2560x1600)
monitor=eDP-1,2560x1600@165,0x0,1.25

# Standard 4K laptop (3840x2160)
monitor=eDP-1,3840x2160@60,0x0,2.0
```

### Power-Efficient Animations

For better battery life:

```conf
decoration {
    blur {
        enabled = false  # Disable blur on battery
    }
}

animations {
    enabled = true
    bezier = easeOut, 0.16, 1, 0.3, 1
    animation = windows, 1, 3, easeOut
    animation = fade, 1, 3, default
}

misc {
    vfr = true  # Variable frame rate saves power
}
```

### Recommended Laptop Keybindings

```conf
# Volume control (laptop function keys)
bind = , XF86AudioRaiseVolume, exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%+
bind = , XF86AudioLowerVolume, exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%-
bind = , XF86AudioMute, exec, wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle

# Brightness control
bind = , XF86MonBrightnessUp, exec, light -A 10
bind = , XF86MonBrightnessDown, exec, light -S 10

# Screen lock for laptop security
bind = SUPER, L, exec, hyprlock
```

### AMD-Specific Hyprland Configuration

For AMD Ryzen laptops (like Framework):

```nix
# AMD GPU optimisation
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

# AMD-specific environment variables
environment.sessionVariables = {
  AMD_VULKAN_ICD = "RADV";
  LIBVA_DRIVER_NAME = "radeonsi";
  mesa_glthread = "true";
};

# AMD GPU power management with TLP
services.tlp.settings = {
  RADEON_DPM_STATE_ON_AC = "performance";
  RADEON_DPM_STATE_ON_BAT = "battery";
  RADEON_POWER_PROFILE_ON_AC = "high";
  RADEON_POWER_PROFILE_ON_BAT = "low";
};

# Monitoring
environment.systemPackages = with pkgs; [
  amdgpu_top  # AMD GPU monitoring
  radeontop
];
```

AMD advantages for Hyprland:

- ✅ Better Wayland support than NVIDIA
- ✅ Hardware cursor works out of the box
- ✅ No screen tearing issues
- ✅ Excellent VRR/FreeSync support
- ✅ Better power management on battery

See `examples/hyprland/hardware/AMD-HYPRLAND.md` for complete AMD configuration.

```

```
