# ZFS Filesystem Configuration

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

Advanced ZFS setup on NixOS for servers and workstations.

## Use Case

ZFS provides:

- Data integrity with checksums
- Snapshots and clones
- Compression
- RAID-Z redundancy
- Native encryption
- Easy backup with send/receive

## Basic ZFS Root Installation

### hardware-configuration.nix

```nix
{ config, lib, pkgs, modulesPath, ... }:

{
  imports = [
    (modulesPath + "/installer/scan/not-detected.nix")
  ];

  boot.initrd.availableKernelModules = [
    "xhci_pci" "ahci" "nvme" "usbhid" "sd_mod"
  ];

  # ZFS support in initrd
  boot.supportedFilesystems = [ "zfs" ];
  boot.zfs.forceImportRoot = false;

  # Required: unique host ID for ZFS
  networking.hostId = "abcd1234";  # Generate with: head -c 8 /etc/machine-id

  # ZFS pools and datasets
  fileSystems."/" = {
    device = "rpool/root";
    fsType = "zfs";
  };

  fileSystems."/home" = {
    device = "rpool/home";
    fsType = "zfs";
  };

  fileSystems."/nix" = {
    device = "rpool/nix";
    fsType = "zfs";
  };

  fileSystems."/var" = {
    device = "rpool/var";
    fsType = "zfs";
  };

  # Boot partition (cannot be ZFS with UEFI)
  fileSystems."/boot" = {
    device = "/dev/disk/by-uuid/XXXX-XXXX";
    fsType = "vfat";
  };

  swapDevices = [ ];  # ZFS doesn't recommend swap on ZFS
}
```

### configuration.nix

```nix
{ config, pkgs, ... }:

{
  # ZFS services
  services.zfs = {
    autoScrub = {
      enable = true;
      interval = "weekly";
    };

    autoSnapshot = {
      enable = true;
      frequent = 4;    # Keep 4 15-minute snapshots
      hourly = 24;     # Keep 24 hourly snapshots
      daily = 7;       # Keep 7 daily snapshots
      weekly = 4;      # Keep 4 weekly snapshots
      monthly = 12;    # Keep 12 monthly snapshots
    };

    trim.enable = true;  # TRIM for SSDs
  };

  # ZFS boot options
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  # ZFS kernel parameters
  boot.kernelParams = [
    "zfs.zfs_arc_max=4294967296"  # Limit ARC to 4GB
  ];
}
```

## Server ZFS Pool Setup

Creating pools for a home server:

### Single Disk Pool

```bash
# Create pool
zpool create -o ashift=12 tank /dev/sda

# Create datasets
zfs create -o mountpoint=/data tank/data
zfs create -o mountpoint=/backup tank/backup
zfs create -o compression=lz4 tank/data/compressed
```

### Mirror Pool (RAID 1)

```bash
# Two-disk mirror
zpool create -o ashift=12 tank mirror /dev/sda /dev/sdb

# Three-way mirror for critical data
zpool create -o ashift=12 critical mirror /dev/sdc /dev/sdd /dev/sde
```

### RAID-Z Pool

```bash
# RAID-Z1 (like RAID 5, single parity)
zpool create -o ashift=12 tank raidz /dev/sda /dev/sdb /dev/sdc /dev/sdd

# RAID-Z2 (like RAID 6, double parity)
zpool create -o ashift=12 tank raidz2 /dev/sda /dev/sdb /dev/sdc /dev/sdd /dev/sde

# RAID-Z3 (triple parity)
zpool create -o ashift=12 tank raidz3 /dev/sd{a,b,c,d,e,f,g}
```

### NixOS Dataset Configuration

```nix
{ config, pkgs, ... }:

{
  # Declare ZFS datasets (created outside Nix, mounted by Nix)
  fileSystems."/data" = {
    device = "tank/data";
    fsType = "zfs";
    options = [ "zfsutil" ];
  };

  fileSystems."/data/media" = {
    device = "tank/data/media";
    fsType = "zfs";
    options = [ "zfsutil" ];
  };

  fileSystems."/data/documents" = {
    device = "tank/data/documents";
    fsType = "zfs";
    options = [ "zfsutil" ];
  };

  fileSystems."/backup" = {
    device = "tank/backup";
    fsType = "zfs";
    options = [ "zfsutil" ];
  };
}
```

## ZFS Native Encryption

```bash
# Create encrypted dataset (prompts for passphrase)
zfs create -o encryption=aes-256-gcm -o keyformat=passphrase tank/encrypted

# Create with key file
zfs create -o encryption=aes-256-gcm -o keyformat=raw \
  -o keylocation=file:///root/zfs.key tank/encrypted-keyfile

# Load key at boot (NixOS)
```

### NixOS Encrypted Dataset

```nix
{ config, pkgs, ... }:

{
  # Load ZFS encryption key at boot
  boot.zfs.requestEncryptionCredentials = [ "tank/encrypted" ];

  # Or use a key file
  # boot.zfs.extraPools = [ "tank" ];

  fileSystems."/encrypted" = {
    device = "tank/encrypted";
    fsType = "zfs";
    options = [ "zfsutil" ];
  };
}
```

## ZFS Snapshots and Backup

### Automatic Snapshots with sanoid

```nix
{ config, pkgs, ... }:

{
  services.sanoid = {
    enable = true;

    datasets = {
      "tank/data" = {
        hourly = 24;
        daily = 30;
        weekly = 4;
        monthly = 12;
        yearly = 1;
        autosnap = true;
        autoprune = true;
      };

      "tank/data/documents" = {
        hourly = 48;  # More frequent for documents
        daily = 60;
        weekly = 8;
        monthly = 24;
        autosnap = true;
        autoprune = true;
      };
    };
  };
}
```

### Remote Backup with syncoid

```nix
{ config, pkgs, ... }:

{
  services.syncoid = {
    enable = true;
    interval = "daily";

    commands = {
      "tank/data" = {
        target = "backup@remote-server:backup/tank-data";
        recursive = true;
        sshKey = "/root/.ssh/id_ed25519_backup";
        extraArgs = [ "--no-sync-snap" ];
      };
    };
  };
}
```

### Manual Snapshot Commands

```bash
# Create snapshot
zfs snapshot tank/data@before-upgrade

# List snapshots
zfs list -t snapshot

# Rollback to snapshot
zfs rollback tank/data@before-upgrade

# Send snapshot to another pool
zfs send tank/data@snap1 | zfs receive backup/data

# Incremental send
zfs send -i tank/data@snap1 tank/data@snap2 | zfs receive backup/data

# Send to remote server
zfs send tank/data@snap1 | ssh backup@server zfs receive backup/data
```

## ZFS Properties and Tuning

```nix
{ config, pkgs, ... }:

{
  # Set ZFS properties via NixOS
  # Note: Most properties should be set when creating datasets

  boot.kernelParams = [
    # Limit ARC (Adaptive Replacement Cache) size
    "zfs.zfs_arc_max=8589934592"  # 8GB
    "zfs.zfs_arc_min=1073741824"  # 1GB

    # Tuning for SSDs
    "zfs.zfs_txg_timeout=5"
  ];

  # Or use modprobe options
  boot.extraModprobeConfig = ''
    options zfs zfs_arc_max=8589934592
  '';
}
```

### Common Dataset Properties

```bash
# Enable compression (lz4 is fast, zstd for better ratio)
zfs set compression=lz4 tank/data
zfs set compression=zstd tank/archive

# Set record size (default 128K, 1M for large files)
zfs set recordsize=1M tank/media

# Enable deduplication (use with caution - RAM intensive)
# zfs set dedup=on tank/backup

# Set quota
zfs set quota=100G tank/data/user1

# Set reservation (guaranteed space)
zfs set reservation=50G tank/data/important

# Disable atime (improves performance)
zfs set atime=off tank/data
```

## Monitoring ZFS

```bash
# Pool status
zpool status
zpool status -v  # Verbose with errors

# Pool I/O stats
zpool iostat 1

# List all datasets
zfs list

# Check space usage
zfs list -o name,used,avail,refer,mountpoint

# Check for errors
zpool scrub tank
zpool status tank  # Check scrub progress

# ZFS events
zpool events -v
```

### NixOS ZFS Monitoring

```nix
{ config, pkgs, ... }:

{
  # ZFS Event Daemon for monitoring
  services.zfs.zed = {
    enableMail = true;
    settings = {
      ZED_EMAIL_ADDR = [ "admin@example.com" ];
      ZED_NOTIFY_VERBOSE = true;
    };
  };

  # Prometheus ZFS exporter
  services.prometheus.exporters.zfs = {
    enable = true;
    port = 9134;
  };
}
```
