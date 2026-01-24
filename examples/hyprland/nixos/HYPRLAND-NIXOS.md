# Hyprland NixOS Integration

Complete NixOS configuration for Hyprland with all essential ecosystem packages.

## Basic Hyprland Setup

```nix
{ config, pkgs, ... }:

{
  # Enable Hyprland
  programs.hyprland = {
    enable = true;
    xwayland.enable = true;
  };

  # Enable XDG desktop portal
  xdg.portal = {
    enable = true;
    extraPortals = with pkgs; [
      xdg-desktop-portal-hyprland
      xdg-desktop-portal-gtk
    ];
  };

  # Essential Hyprland ecosystem packages
  environment.systemPackages = with pkgs; [
    # Hyprland tools
    hyprpaper         # Wallpaper manager
    hyprlock          # Screen locker
    hypridle          # Idle management
    hyprpicker        # Colour picker
    
    # Status bar
    waybar
    
    # Application launcher
    wofi              # Or rofi-wayland
    
    # Notifications
    dunst             # Or mako
    
    # Screenshots
    grim              # Screenshot utility
    slurp             # Screen area selection
    
    # Clipboard
    wl-clipboard      # CLI clipboard utilities
    cliphist          # Clipboard history
    
    # Terminal
    kitty             # Or alacritty, foot, etc.
    
    # File manager
    thunar            # Or nautilus, dolphin
    
    # Image viewer
    imv               # Wayland image viewer
    
    # PDF viewer
    zathura           # Lightweight PDF viewer
    
    # Utilities
    brightnessctl     # Brightness control (alternative to light)
    playerctl         # Media player control
    networkmanagerapplet  # NetworkManager GUI
    blueman           # Bluetooth manager
    pavucontrol       # PulseAudio/PipeWire volume control
  ];

  # Audio support (PipeWire)
  security.rtkit.enable = true;
  services.pipewire = {
    enable = true;
    alsa.enable = true;
    alsa.support32Bit = true;
    pulse.enable = true;
    jack.enable = true;
  };

  # Enable dbus (required for many Wayland applications)
  services.dbus.enable = true;

  # Enable polkit (for privilege escalation)
  security.polkit.enable = true;

  # Graphics drivers - uncomment as needed
  # hardware.opengl = {
  #   enable = true;
  #   driSupport = true;
  #   driSupport32Bit = true;
  # };
}
```

## User-Specific Configuration

Add to your user configuration or home-manager:

```nix
{ config, pkgs, ... }:

{
  home.packages = with pkgs; [
    # Additional user-specific tools
    wev              # Wayland event viewer (for debugging)
    wl-clipboard-rs  # Rust-based clipboard utilities
  ];

  # Home manager can manage your Hyprland config
  # wayland.windowManager.hyprland = {
  #   enable = true;
  #   settings = {
  #     # Your Hyprland configuration here
  #   };
  # };
}
```

## Laptop-Specific NixOS Configuration

```nix
{ config, pkgs, ... }:

{
  # All basic Hyprland setup from above, plus:

  # Brightness control
  programs.light.enable = true;
  users.users.YOUR_USERNAME.extraGroups = [ "video" ];

  # Power management
  services.tlp = {
    enable = true;
    settings = {
      CPU_SCALING_GOVERNOR_ON_AC = "performance";
      CPU_SCALING_GOVERNOR_ON_BAT = "powersave";
      START_CHARGE_THRESH_BAT0 = 40;
      STOP_CHARGE_THRESH_BAT0 = 80;
    };
  };

  # Alternative: power-profiles-daemon
  # services.power-profiles-daemon.enable = true;

  # Touchpad support (libinput)
  services.xserver.libinput = {
    enable = true;
    touchpad = {
      tapping = true;
      naturalScrolling = true;
      disableWhileTyping = true;
    };
  };

  # WiFi and Bluetooth
  networking.networkmanager.enable = true;
  hardware.bluetooth = {
    enable = true;
    powerOnBoot = true;
  };
  services.blueman.enable = true;

  # Laptop-specific packages
  environment.systemPackages = with pkgs; [
    acpi              # Battery info
    powertop          # Power usage monitor
    tlp               # Power management
  ];
}
```

## Intel GPU Configuration

```nix
{ config, pkgs, ... }:

{
  # Intel GPU hardware acceleration
  hardware.opengl = {
    enable = true;
    extraPackages = with pkgs; [
      intel-media-driver    # VAAPI for newer Intel GPUs (Broadwell+)
      vaapiIntel            # VAAPI for older Intel GPUs
      vaapiVdpau
      libvdpau-va-gl
      intel-compute-runtime # OpenCL
    ];
  };

  # Intel-specific environment variables for Hyprland
  environment.sessionVariables = {
    LIBVA_DRIVER_NAME = "iHD";  # Use iHD for newer Intel GPUs
    # LIBVA_DRIVER_NAME = "i965";  # Use i965 for older Intel GPUs
  };
}
```

## AMD GPU Configuration

```nix
{ config, pkgs, ... }:

{
  # AMD GPU hardware acceleration
  hardware.opengl = {
    enable = true;
    extraPackages = with pkgs; [
      rocm-opencl-icd
      rocm-opencl-runtime
      amdvlk
    ];
    extraPackages32 = with pkgs; [
      driversi686Linux.amdvlk
    ];
  };

  # AMD-specific drivers
  boot.initrd.kernelModules = [ "amdgpu" ];
  services.xserver.videoDrivers = [ "amdgpu" ];

  # AMD-specific environment variables
  environment.sessionVariables = {
    ROC_ENABLE_PRE_VEGA = "1";
  };
}
```

## NVIDIA GPU Configuration

```nix
{ config, pkgs, ... }:

{
  # NVIDIA drivers
  services.xserver.videoDrivers = [ "nvidia" ];
  
  hardware.nvidia = {
    modesetting.enable = true;
    powerManagement.enable = true;
    open = false;  # Use proprietary drivers
    nvidiaSettings = true;
    package = config.boot.kernelPackages.nvidiaPackages.stable;
  };

  hardware.opengl = {
    enable = true;
    driSupport = true;
    driSupport32Bit = true;
  };

  # NVIDIA-specific environment variables for Hyprland
  environment.sessionVariables = {
    LIBVA_DRIVER_NAME = "nvidia";
    XDG_SESSION_TYPE = "wayland";
    GBM_BACKEND = "nvidia-drm";
    __GLX_VENDOR_LIBRARY_NAME = "nvidia";
    WLR_NO_HARDWARE_CURSORS = "1";
  };
}
```

## Display Manager Configuration

### GDM (GNOME Display Manager)

```nix
{
  services.xserver = {
    enable = true;
    displayManager.gdm = {
      enable = true;
      wayland = true;
    };
  };
}
```

### SDDM (Simple Desktop Display Manager)

```nix
{
  services.xserver = {
    enable = true;
    displayManager.sddm = {
      enable = true;
      wayland.enable = true;
    };
  };
}
```

### TTY Autologin (No Display Manager)

```nix
{
  services.greetd = {
    enable = true;
    settings = {
      default_session = {
        command = "${pkgs.greetd.tuigreet}/bin/tuigreet --time --cmd Hyprland";
        user = "greeter";
      };
    };
  };

  # Alternative: Auto-login to TTY and start Hyprland
  services.getty.autologinUser = "YOUR_USERNAME";
  
  # Add to shell profile (.bashrc or .zshrc):
  # if [ -z "$WAYLAND_DISPLAY" ] && [ "$XDG_VTNR" -eq 1 ]; then
  #   exec Hyprland
  # fi
}
```

## Complete Example Configuration

```nix
{ config, pkgs, ... }:

{
  # Enable Hyprland
  programs.hyprland = {
    enable = true;
    xwayland.enable = true;
  };

  # XDG portal
  xdg.portal = {
    enable = true;
    extraPortals = with pkgs; [
      xdg-desktop-portal-hyprland
      xdg-desktop-portal-gtk
    ];
  };

  # Display manager
  services.xserver = {
    enable = true;
    displayManager.gdm = {
      enable = true;
      wayland = true;
    };
  };

  # Audio
  security.rtkit.enable = true;
  services.pipewire = {
    enable = true;
    alsa.enable = true;
    pulse.enable = true;
  };

  # Graphics (Intel example)
  hardware.opengl = {
    enable = true;
    extraPackages = with pkgs; [
      intel-media-driver
      vaapiIntel
    ];
  };

  # Networking and Bluetooth
  networking.networkmanager.enable = true;
  hardware.bluetooth.enable = true;
  services.blueman.enable = true;

  # Essential packages
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
    
    # Terminal and utilities
    kitty
    thunar
    brightnessctl
    playerctl
    networkmanagerapplet
    blueman
    pavucontrol
  ];

  # Fonts (important for waybar icons)
  fonts.packages = with pkgs; [
    noto-fonts
    noto-fonts-emoji
    font-awesome
    (nerdfonts.override { fonts = [ "JetBrainsMono" "FiraCode" ]; })
  ];

  # User configuration
  users.users.YOUR_USERNAME = {
    isNormalUser = true;
    extraGroups = [ "wheel" "networkmanager" "video" "audio" ];
  };
}
```

## Home Manager Integration

If using Home Manager, you can manage Hyprland configuration declaratively:

```nix
{ config, pkgs, ... }:

{
  wayland.windowManager.hyprland = {
    enable = true;
    settings = {
      "$mod" = "SUPER";
      
      monitor = [
        ",preferred,auto,1"
      ];
      
      exec-once = [
        "waybar"
        "dunst"
        "hyprpaper"
      ];
      
      general = {
        gaps_in = 5;
        gaps_out = 10;
        border_size = 2;
      };
      
      decoration = {
        rounding = 8;
      };
      
      bind = [
        "$mod, Q, exec, kitty"
        "$mod, C, killactive"
        "$mod, M, exit"
        "$mod, R, exec, wofi --show drun"
        # Add all your keybindings here
      ];
    };
  };

  # Waybar configuration
  programs.waybar = {
    enable = true;
    # settings and style can be configured here
  };
}
```

## Troubleshooting

### Hyprland Won't Start

Check journal logs:
```bash
journalctl -xe | grep -i hyprland
```

### Screen Tearing

Enable VRR or adjust `vfr` setting.

### NVIDIA Cursor Issues

Set `WLR_NO_HARDWARE_CURSORS=1` in environment variables (shown in NVIDIA section above).

### Missing Icons in Waybar

Install nerd fonts:
```nix
fonts.packages = with pkgs; [
  (nerdfonts.override { fonts = [ "JetBrainsMono" ]; })
];
```
