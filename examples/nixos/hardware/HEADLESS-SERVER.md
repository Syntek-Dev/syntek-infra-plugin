# Headless Server Hardware Configuration

Hardware configuration for servers without GPU/display output.

## Use Case

Servers running without graphics:

- Home servers (NAS, media, home automation)
- Cloud VMs and VPS instances
- Dedicated servers in datacentres
- Raspberry Pi and ARM SBCs

## Basic Headless Server (x86_64)

### hardware-configuration.nix

```nix
{ config, lib, pkgs, modulesPath, ... }:

{
  imports = [
    (modulesPath + "/installer/scan/not-detected.nix")
  ];

  # Boot - no graphics needed
  boot.initrd.availableKernelModules = [
    "ahci"
    "xhci_pci"
    "nvme"
    "usbhid"
    "sd_mod"
  ];
  boot.initrd.kernelModules = [ ];
  boot.kernelModules = [ "kvm-intel" ];  # or kvm-amd
  boot.extraModulePackages = [ ];

  # Filesystems
  fileSystems."/" = {
    device = "/dev/disk/by-uuid/YOUR-ROOT-UUID";
    fsType = "ext4";
  };

  fileSystems."/boot" = {
    device = "/dev/disk/by-uuid/YOUR-BOOT-UUID";
    fsType = "vfat";
  };

  # Data drives (example)
  fileSystems."/data" = {
    device = "/dev/disk/by-uuid/YOUR-DATA-UUID";
    fsType = "ext4";
    options = [ "nofail" ];  # Don't fail boot if missing
  };

  swapDevices = [
    { device = "/dev/disk/by-uuid/YOUR-SWAP-UUID"; }
  ];

  # Platform
  nixpkgs.hostPlatform = lib.mkDefault "x86_64-linux";
}
```

### configuration.nix

```nix
{ config, pkgs, ... }:

{
  # No GUI
  services.xserver.enable = false;

  # Text console only
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  # Console configuration
  console = {
    font = "Lat2-Terminus16";
    keyMap = "uk";
  };

  # Serial console (useful for cloud/VM)
  boot.kernelParams = [
    "console=ttyS0,115200"
    "console=tty1"
  ];

  # Networking
  networking.hostName = "server";
  networking.networkmanager.enable = false;  # Use systemd-networkd instead

  systemd.network = {
    enable = true;
    networks."10-lan" = {
      matchConfig.Name = "en*";
      networkConfig = {
        DHCP = "yes";
        # Or static:
        # Address = "192.168.1.10/24";
        # Gateway = "192.168.1.1";
        # DNS = "1.1.1.1";
      };
    };
  };

  # SSH - primary access method
  services.openssh = {
    enable = true;
    settings = {
      PermitRootLogin = "prohibit-password";
      PasswordAuthentication = false;
    };
  };

  # Users
  users.users.admin = {
    isNormalUser = true;
    extraGroups = [ "wheel" ];
    openssh.authorizedKeys.keys = [
      "ssh-ed25519 AAAA... your-key"
    ];
  };

  # Minimal packages
  environment.systemPackages = with pkgs; [
    vim
    git
    htop
    tmux
    curl
    wget
  ];

  # Enable flakes
  nix.settings.experimental-features = [ "nix-command" "flakes" ];

  system.stateVersion = "24.05";
}
```

## Cloud VM Configuration

For Hetzner, AWS, DigitalOcean, etc.:

```nix
{ config, pkgs, modulesPath, ... }:

{
  imports = [
    (modulesPath + "/profiles/qemu-guest.nix")
  ];

  boot.loader.grub = {
    enable = true;
    device = "/dev/sda";  # Or /dev/vda for virtio
  };

  # Virtio drivers
  boot.initrd.availableKernelModules = [
    "virtio_pci"
    "virtio_scsi"
    "virtio_blk"
    "virtio_net"
  ];

  # Cloud-init compatible
  services.cloud-init.enable = true;

  # Or use Hetzner-specific module
  # imports = [ (modulesPath + "/profiles/hetzner.nix") ];

  # Filesystem (typically single disk)
  fileSystems."/" = {
    device = "/dev/sda1";  # Or /dev/vda1
    fsType = "ext4";
  };

  # Networking via DHCP
  networking.useDHCP = true;

  # No swap on cloud (usually)
  swapDevices = [ ];

  # Timezone
  time.timeZone = "UTC";
}
```

## Raspberry Pi (ARM64)

```nix
{ config, pkgs, modulesPath, ... }:

{
  imports = [
    (modulesPath + "/installer/sd-card/sd-image-aarch64.nix")
  ];

  # Platform
  nixpkgs.hostPlatform = "aarch64-linux";

  # Raspberry Pi specific
  hardware.raspberry-pi."4".enable = true;

  # Boot
  boot.loader.grub.enable = false;
  boot.loader.generic-extlinux-compatible.enable = true;

  # Kernel
  boot.kernelPackages = pkgs.linuxPackages_rpi4;

  # Filesystem
  fileSystems."/" = {
    device = "/dev/disk/by-label/NIXOS_SD";
    fsType = "ext4";
  };

  # Enable SSH on first boot
  services.openssh.enable = true;

  # WiFi (if using)
  networking.wireless.enable = true;

  # Or Ethernet
  networking.interfaces.eth0.useDHCP = true;
}
```

## Server Optimisations

```nix
{ config, pkgs, ... }:

{
  # Disable suspend/hibernate
  services.logind.extraConfig = ''
    HandleSuspendKey=ignore
    HandleHibernateKey=ignore
    HandleLidSwitch=ignore
  '';

  # Performance governor
  powerManagement.cpuFreqGovernor = "performance";

  # Kernel parameters for servers
  boot.kernel.sysctl = {
    # Network performance
    "net.core.somaxconn" = 65535;
    "net.core.netdev_max_backlog" = 65535;
    "net.ipv4.tcp_max_syn_backlog" = 65535;
    "net.ipv4.tcp_fastopen" = 3;

    # Memory
    "vm.swappiness" = 10;
    "vm.dirty_ratio" = 60;
    "vm.dirty_background_ratio" = 2;

    # File handles
    "fs.file-max" = 2097152;
    "fs.inotify.max_user_watches" = 524288;
  };

  # Increase limits
  security.pam.loginLimits = [
    { domain = "*"; type = "soft"; item = "nofile"; value = "65536"; }
    { domain = "*"; type = "hard"; item = "nofile"; value = "65536"; }
  ];

  # Disable unnecessary services
  services.avahi.enable = false;
  services.printing.enable = false;
  sound.enable = false;
  hardware.pulseaudio.enable = false;

  # Automatic updates
  system.autoUpgrade = {
    enable = true;
    flake = "/etc/nixos";
    flags = [ "--update-input" "nixpkgs" ];
    dates = "04:00";
    allowReboot = false;  # Set true for automatic reboots
  };

  # Garbage collection
  nix.gc = {
    automatic = true;
    dates = "weekly";
    options = "--delete-older-than 30d";
  };
}
```

## Remote Management

```nix
{ config, pkgs, ... }:

{
  # SSH with hardened config
  services.openssh = {
    enable = true;
    ports = [ 22 ];
    settings = {
      PermitRootLogin = "prohibit-password";
      PasswordAuthentication = false;
      KbdInteractiveAuthentication = false;
      X11Forwarding = false;
      AllowTcpForwarding = true;
      AllowAgentForwarding = true;
    };
  };

  # Mosh for unstable connections
  programs.mosh.enable = true;

  # Fail2ban
  services.fail2ban = {
    enable = true;
    maxretry = 5;
    bantime = "1h";
  };

  # Monitoring
  services.prometheus.exporters.node = {
    enable = true;
    port = 9100;
    enabledCollectors = [
      "cpu"
      "diskstats"
      "filesystem"
      "loadavg"
      "meminfo"
      "netdev"
      "systemd"
    ];
  };
}
```

## IPMI/BMC Access (Enterprise Servers)

```nix
{ config, pkgs, ... }:

{
  # IPMI tools
  environment.systemPackages = with pkgs; [
    ipmitool
    freeipmi
  ];

  # Load IPMI kernel modules
  boot.kernelModules = [
    "ipmi_devintf"
    "ipmi_si"
  ];

  # IPMI monitoring (for Prometheus)
  services.prometheus.exporters.ipmi = {
    enable = true;
    port = 9290;
  };
}
```
