# AMD Laptop Hardware Configuration

Hardware configuration patterns for AMD-based laptops (Ryzen CPUs with
integrated graphics).

## Use Case

Laptops with AMD Ryzen CPU and integrated Radeon graphics (ThinkPad T14s AMD,
Framework AMD, etc.).

## hardware-configuration.nix

```nix
{ config, lib, pkgs, modulesPath, ... }:

{
  imports = [
    (modulesPath + "/installer/scan/not-detected.nix")
  ];

  # Boot
  boot.initrd.availableKernelModules = [
    "nvme"
    "xhci_pci"
    "ahci"
    "usb_storage"
    "sd_mod"
  ];
  boot.initrd.kernelModules = [ ];
  boot.kernelModules = [ "kvm-amd" ];
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
  hardware.cpu.amd.updateMicrocode =
    lib.mkDefault config.hardware.enableRedistributableFirmware;
}
```

## AMD Integrated Graphics (RDNA/Vega)

```nix
{ config, pkgs, ... }:

{
  # AMD GPU support
  hardware.graphics = {
    enable = true;
    enable32Bit = true;

    extraPackages = with pkgs; [
      amdvlk           # AMD Vulkan driver
      rocmPackages.clr # OpenCL
    ];

    extraPackages32 = with pkgs; [
      driversi686Linux.amdvlk
    ];
  };

  # Use AMDGPU kernel driver
  boot.initrd.kernelModules = [ "amdgpu" ];

  # Video acceleration
  environment.sessionVariables = {
    # Use RADV (Mesa) Vulkan driver - generally better than AMDVLK
    AMD_VULKAN_ICD = "RADV";
  };
}
```

## Power Management for AMD

```nix
{ config, pkgs, ... }:

{
  # AMD-specific power management
  services.power-profiles-daemon.enable = true;

  # Or use TLP for more control
  # services.tlp = {
  #   enable = true;
  #   settings = {
  #     CPU_SCALING_GOVERNOR_ON_AC = "performance";
  #     CPU_SCALING_GOVERNOR_ON_BAT = "powersave";
  #
  #     # AMD-specific
  #     CPU_BOOST_ON_AC = 1;
  #     CPU_BOOST_ON_BAT = 0;
  #
  #     # Platform profile
  #     PLATFORM_PROFILE_ON_AC = "performance";
  #     PLATFORM_PROFILE_ON_BAT = "low-power";
  #
  #     # Battery thresholds (if supported)
  #     START_CHARGE_THRESH_BAT0 = 40;
  #     STOP_CHARGE_THRESH_BAT0 = 80;
  #   };
  # };

  # AMD P-State driver (for Zen 2+)
  boot.kernelParams = [
    "amd_pstate=active"  # Use AMD P-State EPP driver
  ];

  # Suspend/resume
  services.logind = {
    lidSwitch = "suspend";
    extraConfig = ''
      HandlePowerKey=suspend
    '';
  };

  # Enable S3 sleep if available
  boot.kernelParams = [
    "mem_sleep_default=deep"
  ];
}
```

## AMD Sensor Monitoring

```nix
{ config, pkgs, ... }:

{
  # Load AMD-specific sensor modules
  boot.kernelModules = [ "zenpower" ];  # Better than k10temp for Zen
  boot.extraModulePackages = with config.boot.kernelPackages; [
    zenpower
  ];

  # Or use standard k10temp
  # boot.kernelModules = [ "k10temp" ];

  # Monitoring tools
  environment.systemPackages = with pkgs; [
    lm_sensors
    zenmonitor   # AMD Zen CPU monitor
    radeontop    # AMD GPU monitor
  ];
}
```

## WiFi and Bluetooth

```nix
{ config, pkgs, ... }:

{
  # WiFi (common AMD laptop chips: MediaTek, Intel, Realtek)
  networking.networkmanager.enable = true;

  # For MediaTek WiFi (common in AMD laptops)
  hardware.firmware = [ pkgs.linux-firmware ];

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

## Firmware Updates

```nix
{ config, pkgs, ... }:

{
  # Enable firmware updates
  services.fwupd.enable = true;

  # AMD firmware
  hardware.enableRedistributableFirmware = true;

  # Specific AMD firmware packages
  hardware.firmware = with pkgs; [
    linux-firmware
    # amd-ucode is handled by hardware.cpu.amd.updateMicrocode
  ];
}
```

## Fingerprint Reader

```nix
{ config, pkgs, ... }:

{
  # Fingerprint (common: Goodix, Synaptics)
  services.fprintd.enable = true;

  # PAM integration
  security.pam.services.login.fprintAuth = true;
  security.pam.services.sudo.fprintAuth = true;
}
```

## Framework Laptop (AMD) Specific

```nix
{ config, pkgs, ... }:

{
  # Framework-specific settings
  hardware.framework.enable = true;

  # Ambient light sensor
  hardware.sensor.iio.enable = true;

  # Function key behaviour
  boot.kernelParams = [
    "module_blacklist=hid_sensor_hub"  # Fix fn keys on some models
  ];
}
```
