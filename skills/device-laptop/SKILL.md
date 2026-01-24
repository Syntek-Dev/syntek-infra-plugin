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
