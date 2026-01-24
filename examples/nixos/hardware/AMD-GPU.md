# AMD Discrete GPU Configuration

Hardware configuration for systems with AMD discrete graphics cards (RX
5000/6000/7000 series).

## Use Case

Workstations and gaming systems with:

- AMD Radeon RX discrete GPUs
- AMD + AMD (CPU + GPU) configurations
- Intel + AMD (CPU + GPU) configurations

## Basic AMD GPU Setup

```nix
{ config, pkgs, ... }:

{
  # Enable AMD GPU driver
  boot.initrd.kernelModules = [ "amdgpu" ];

  # Graphics configuration
  hardware.graphics = {
    enable = true;
    enable32Bit = true;  # For 32-bit applications/games

    extraPackages = with pkgs; [
      # Vulkan
      amdvlk

      # Video acceleration
      vaapiVdpau
      libvdpau-va-gl

      # OpenCL (ROCm)
      rocmPackages.clr
      rocmPackages.clr.icd
    ];

    extraPackages32 = with pkgs; [
      driversi686Linux.amdvlk
    ];
  };

  # Use Mesa RADV Vulkan driver (recommended over AMDVLK for most uses)
  environment.sessionVariables = {
    AMD_VULKAN_ICD = "RADV";
  };
}
```

## AMD GPU with Variable Refresh Rate (FreeSync)

```nix
{ config, pkgs, ... }:

{
  hardware.graphics.enable = true;
  boot.initrd.kernelModules = [ "amdgpu" ];

  # Enable FreeSync/VRR
  services.xserver.deviceSection = ''
    Option "VariableRefresh" "true"
  '';

  # For Wayland compositors (Sway, Hyprland)
  # VRR is typically enabled in compositor config
}
```

## AMD GPU Overclocking and Fan Control

```nix
{ config, pkgs, ... }:

{
  # Allow GPU parameter changes
  boot.kernelParams = [
    "amdgpu.ppfeaturemask=0xffffffff"  # Enable all power features
  ];

  # Tools for GPU control
  environment.systemPackages = with pkgs; [
    corectrl      # GUI for AMD GPU control
    radeontop     # GPU monitoring
    nvtopPackages.amd  # GPU process monitor
  ];

  # Allow users in wheel group to control GPU
  programs.corectrl = {
    enable = true;
    gpuOverclock.enable = true;
  };

  # Polkit rule for corectrl
  security.polkit.extraConfig = ''
    polkit.addRule(function(action, subject) {
      if ((action.id == "org.corectrl.helper.init" ||
           action.id == "org.corectrl.helperkiller.init") &&
          subject.local == true &&
          subject.active == true &&
          subject.isInGroup("wheel")) {
        return polkit.Result.YES;
      }
    });
  '';
}
```

## ROCm for Machine Learning/Compute

```nix
{ config, pkgs, ... }:

{
  hardware.graphics = {
    enable = true;
    extraPackages = with pkgs; [
      rocmPackages.clr
      rocmPackages.clr.icd
      rocmPackages.rocm-runtime
    ];
  };

  # ROCm environment
  systemd.tmpfiles.rules = [
    "L+    /opt/rocm/hip   -    -    -     -    ${pkgs.rocmPackages.clr}"
  ];

  # Add user to video and render groups
  users.users.myuser = {
    extraGroups = [ "video" "render" ];
  };

  # Environment variables for ROCm
  environment.sessionVariables = {
    HIP_PLATFORM = "amd";
    # For specific GPU architectures (check with rocminfo)
    # HSA_OVERRIDE_GFX_VERSION = "10.3.0";  # Example for RX 6000 series
  };

  # ROCm tools
  environment.systemPackages = with pkgs; [
    rocmPackages.rocminfo
    rocmPackages.rocm-smi
    rocmPackages.hip-common
  ];
}
```

## Multi-GPU Configuration

### AMD + AMD (Two Discrete GPUs)

```nix
{ config, pkgs, ... }:

{
  boot.initrd.kernelModules = [ "amdgpu" ];

  hardware.graphics = {
    enable = true;
    enable32Bit = true;
    extraPackages = with pkgs; [
      amdvlk
      rocmPackages.clr
    ];
  };

  # Set primary GPU via kernel parameter if needed
  # boot.kernelParams = [ "amdgpu.pcie_gen_cap=0x40000" ];

  # Or use Xorg config to select primary
  services.xserver.deviceSection = ''
    Option "PrimaryGPU" "yes"
  '';
}
```

### AMD + Integrated (Hybrid Graphics)

```nix
{ config, pkgs, ... }:

{
  boot.initrd.kernelModules = [ "amdgpu" ];

  hardware.graphics = {
    enable = true;
    enable32Bit = true;
    extraPackages = with pkgs; [ amdvlk ];
  };

  # PRIME render offload for hybrid graphics
  # Run applications on discrete GPU with:
  # DRI_PRIME=1 application

  environment.sessionVariables = {
    # Default to integrated graphics
    DRI_PRIME = "0";
  };

  # Convenience wrapper
  environment.systemPackages = [
    (pkgs.writeShellScriptBin "prime-run" ''
      DRI_PRIME=1 exec "$@"
    '')
  ];
}
```

## Power Management for Discrete AMD GPU

```nix
{ config, pkgs, ... }:

{
  boot.kernelParams = [
    # Enable runtime power management
    "amdgpu.runpm=1"

    # Dynamic power management state
    "amdgpu.dpm=1"
  ];

  # Runtime power management
  services.udev.extraRules = ''
    # Enable runtime PM for AMD GPU
    ACTION=="add", SUBSYSTEM=="pci", ATTR{vendor}=="0x1002", ATTR{class}=="0x030000", ATTR{power/control}="auto"
  '';
}
```

## Troubleshooting AMD GPU

```bash
# Check GPU is detected
lspci -k | grep -A 3 VGA

# Check AMDGPU driver is loaded
lsmod | grep amdgpu

# GPU information
cat /sys/class/drm/card0/device/gpu_busy_percent
cat /sys/class/drm/card0/device/hwmon/hwmon*/temp1_input

# ROCm detection
rocminfo

# Vulkan check
vulkaninfo | grep -A 5 "GPU"

# Monitor GPU
radeontop
watch -n 1 cat /sys/class/drm/card0/device/gpu_busy_percent
```

## Common Issues

### Screen Tearing

```nix
{
  # For X11
  services.xserver.deviceSection = ''
    Option "TearFree" "true"
  '';

  # For Wayland - handled by compositor
}
```

### GPU Reset on Wake

```nix
{
  boot.kernelParams = [
    "amdgpu.gpu_recovery=1"
  ];
}
```

### Black Screen After Driver Update

```nix
{
  # Force specific firmware version
  hardware.firmware = [
    (pkgs.linux-firmware.overrideAttrs (old: {
      # Pin to specific version if needed
    }))
  ];
}
```
