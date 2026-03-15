# Intel Laptop Hardware Configuration

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

Hardware configuration patterns for Intel-based laptops.

## Use Case

Laptops with Intel CPU and integrated graphics (common ThinkPads, XPS, etc.).

## hardware-configuration.nix

```nix
{ config, lib, pkgs, modulesPath, ... }:

{
  imports = [
    (modulesPath + "/installer/scan/not-detected.nix")
  ];

  # Boot
  boot.initrd.availableKernelModules = [
    "xhci_pci"
    "thunderbolt"
    "nvme"
    "usb_storage"
    "sd_mod"
  ];
  boot.initrd.kernelModules = [ ];
  boot.kernelModules = [ "kvm-intel" ];
  boot.extraModulePackages = [ ];

  # Filesystems (adjust UUIDs for your system)
  fileSystems."/" = {
    device = "/dev/disk/by-uuid/YOUR-ROOT-UUID";
    fsType = "ext4";
  };

  fileSystems."/boot" = {
    device = "/dev/disk/by-uuid/YOUR-BOOT-UUID";
    fsType = "vfat";
  };

  swapDevices = [
    { device = "/dev/disk/by-uuid/YOUR-SWAP-UUID"; }
  ];

  # Platform
  nixpkgs.hostPlatform = lib.mkDefault "x86_64-linux";
  hardware.cpu.intel.updateMicrocode =
    lib.mkDefault config.hardware.enableRedistributableFirmware;
}
```

## Intel Graphics

```nix
# In configuration.nix
{ config, pkgs, ... }:

{
  # Intel GPU support
  hardware.opengl = {
    enable = true;
    driSupport = true;
    driSupport32Bit = true;

    extraPackages = with pkgs; [
      intel-media-driver    # VAAPI for Broadwell+ (2015+)
      vaapiIntel           # VAAPI for older Intel (pre-2015)
      vaapiVdpau
      libvdpau-va-gl
      intel-compute-runtime # OpenCL
    ];
  };

  # Video acceleration
  environment.sessionVariables = {
    LIBVA_DRIVER_NAME = "iHD";  # Use intel-media-driver
  };
}
```

## Power Management

```nix
{
  # TLP for battery optimisation
  services.tlp = {
    enable = true;
    settings = {
      # CPU governor
      CPU_SCALING_GOVERNOR_ON_AC = "performance";
      CPU_SCALING_GOVERNOR_ON_BAT = "powersave";

      # Intel CPU specific
      CPU_ENERGY_PERF_POLICY_ON_AC = "performance";
      CPU_ENERGY_PERF_POLICY_ON_BAT = "power";

      # Battery charge thresholds (if supported)
      START_CHARGE_THRESH_BAT0 = 40;
      STOP_CHARGE_THRESH_BAT0 = 80;

      # WiFi power saving
      WIFI_PWR_ON_AC = "off";
      WIFI_PWR_ON_BAT = "on";
    };
  };

  # Thermald for Intel thermal management
  services.thermald.enable = true;

  # Power button behaviour
  services.logind = {
    lidSwitch = "suspend";
    extraConfig = ''
      HandlePowerKey=suspend
    '';
  };
}
```

## Thunderbolt

```nix
{
  # Thunderbolt support
  services.hardware.bolt.enable = true;

  # Kernel modules for Thunderbolt
  boot.kernelModules = [ "thunderbolt" ];
}
```

## HiDPI Display

```nix
{
  # For 4K or high-DPI displays
  services.xserver.dpi = 192;  # Adjust based on your display

  # Or let the desktop environment handle it
  # services.xserver.dpi = 96;  # Default, DE handles scaling

  # Console font for HiDPI
  console = {
    font = "ter-i32b";
    packages = [ pkgs.terminus_font ];
  };
}
```

## Firmware Updates

```nix
{
  # Enable firmware updates via fwupd
  services.fwupd.enable = true;

  # Allow redistributable firmware
  hardware.enableRedistributableFirmware = true;
}
```

## Fingerprint Reader (if present)

```nix
{
  # Fingerprint authentication
  services.fprintd.enable = true;

  # PAM configuration for fingerprint
  security.pam.services.login.fprintAuth = true;
  security.pam.services.sudo.fprintAuth = true;
}
```

## WiFi and Bluetooth

```nix
{
  # WiFi via NetworkManager
  networking.networkmanager.enable = true;

  # Bluetooth
  hardware.bluetooth = {
    enable = true;
    powerOnBoot = true;
    settings = {
      General = {
        Enable = "Source,Sink,Media,Socket";
      };
    };
  };
  services.blueman.enable = true;
}
```
