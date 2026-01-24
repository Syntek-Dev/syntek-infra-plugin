# Device: Mobile (Pinephone/Pinetab)

This skill provides guidance for configuring NixOS on mobile devices like
Pinephone and Pinetab.

## Important Notes

Mobile NixOS support is still maturing. Some features may require:

- Community packages from `nixos-mobile`
- Manual hardware configuration
- Testing on actual hardware

## Hardware Support

### Pinephone

```nix
imports = [
  # Import mobile-nixos modules
  (builtins.fetchTarball {
    url = "https://github.com/NixOS/mobile-nixos/archive/master.tar.gz";
  } + "/devices/pine64-pinephone")
];

# Pinephone-specific settings
mobile.device.name = "pine64-pinephone";
mobile.device.identity = {
  name = "Pinephone";
  manufacturer = "Pine64";
};
```

### Pinetab

```nix
imports = [
  (builtins.fetchTarball {
    url = "https://github.com/NixOS/mobile-nixos/archive/master.tar.gz";
  } + "/devices/pine64-pinetab")
];

mobile.device.name = "pine64-pinetab";
```

## Mobile Environment

### Phosh (GNOME-based)

```nix
# Phosh mobile shell
services.xserver.desktopManager.phosh = {
  enable = true;
  user = "mobile";
  group = "users";
};

# Required services
services.gnome.gnome-keyring.enable = true;
programs.calls.enable = true;
```

### Plasma Mobile (Alternative)

```nix
services.xserver.desktopManager.plasma5.mobile.enable = true;
```

## Core Configuration

### User Setup

```nix
users.users.mobile = {
  isNormalUser = true;
  extraGroups = [ "wheel" "networkmanager" "video" "audio" "dialout" ];
  initialPassword = "changeme";  # Change on first boot
};
```

### Networking

```nix
networking = {
  hostName = "pinephone";
  networkmanager.enable = true;
  wireless.enable = false;  # Use NetworkManager
};

# Modem support
services.modemmanager.enable = true;
```

### Power Management

```nix
# Suspend on lid close / power button
services.logind = {
  lidSwitch = "suspend";
  extraConfig = ''
    HandlePowerKey=suspend
  '';
};

# Auto-suspend
powerManagement = {
  enable = true;
  powertop.enable = true;
};
```

## Wireguard Configuration

### Road Warrior Client

Mobile devices typically use road warrior configuration:

```nix
networking.wireguard.interfaces.wg0 = {
  ips = [ "10.100.0.3/24" ];
  privateKeyFile = "/run/secrets/wireguard-mobile-key";

  peers = [{
    publicKey = "HOME_SERVER_PUBLIC_KEY";
    allowedIPs = [ "10.100.0.0/24" "192.168.1.0/24" ];
    endpoint = "home.example.com:51820";
    persistentKeepalive = 25;  # Important for mobile
  }];
};
```

### Mullvad VPN

For privacy when on public networks:

```nix
networking.wireguard.interfaces.mullvad = {
  ips = [ "10.66.x.x/32" ];
  privateKeyFile = "/run/secrets/mullvad-mobile-key";

  peers = [{
    publicKey = "MULLVAD_PUBLIC_KEY";
    allowedIPs = [ "0.0.0.0/0" "::/0" ];
    endpoint = "MULLVAD_SERVER:51820";
  }];
};
```

## Mobile Applications

### Essential Apps

```nix
environment.systemPackages = with pkgs; [
  # Communication
  chatty          # SMS/MMS
  calls           # Phone calls
  geary           # Email

  # Web
  epiphany        # Web browser (touch-friendly)

  # Utilities
  gnome.gnome-contacts
  gnome.gnome-calendar
  gnome.gnome-maps

  # File management
  gnome.nautilus

  # Camera
  megapixels
];
```

### Flatpak for Additional Apps

```nix
services.flatpak.enable = true;

# Add Flathub
system.activationScripts.flatpak = ''
  ${pkgs.flatpak}/bin/flatpak remote-add --if-not-exists \
    flathub https://flathub.org/repo/flathub.flatpakrepo
'';
```

## Hardware Features

### Camera

```nix
# Camera support
environment.systemPackages = [ pkgs.megapixels ];

# Grant video access
users.users.mobile.extraGroups = [ "video" ];
```

### GPS

```nix
services.geoclue2 = {
  enable = true;
  enableDemoAgent = false;
};
```

### Sensors

```nix
# Accelerometer, light sensor, etc.
hardware.sensor.iio.enable = true;
```

## Profile Options

### Minimal Profile

- Basic mobile shell (Phosh)
- Phone and SMS
- WiFi networking
- Basic apps

### Full Profile

- Everything in minimal
- Wireguard VPN (home access + Mullvad)
- Full app suite
- Flatpak support
- Camera and GPS

## Limitations

Current limitations to be aware of:

1. **Battery life** - May not be as optimised as mainline mobile OS
2. **Camera quality** - Still being improved in megapixels
3. **App availability** - Limited compared to Android/iOS
4. **Hardware support** - Some features may not work perfectly

## Development Notes

For development on mobile:

```nix
# SSH access for debugging
services.openssh.enable = true;

# ADB-like access
programs.adb.enable = true;
```

## Testing

Before deploying to actual hardware:

1. Test in QEMU with mobile-nixos emulation
2. Verify all services start correctly
3. Test Wireguard connectivity
4. Test basic calling/SMS functionality
