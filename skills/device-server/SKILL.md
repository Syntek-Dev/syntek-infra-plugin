# Device: Home/Business Server

This skill provides guidance for configuring NixOS on home or business servers.

## Server Role

The home/business server acts as:

- Wireguard VPN hub (full tunnel for all devices)
- Docker host for services
- Hashicorp Vault server (self-hosted)
- Backup destination
- Monitoring hub

## Hardware Considerations

### Storage

```nix
# ZFS for data integrity (recommended)
boot.supportedFilesystems = [ "zfs" ];
services.zfs = {
  autoScrub.enable = true;
  autoSnapshot.enable = true;
};

# Or simple ext4 with RAID
boot.swraid.enable = true;
```

### Networking

```nix
# Static IP (recommended for servers)
networking = {
  hostName = "home-server";
  useDHCP = false;
  interfaces.enp0s3 = {
    ipv4.addresses = [{
      address = "192.168.1.10";
      prefixLength = 24;
    }];
  };
  defaultGateway = "192.168.1.1";
  nameservers = [ "1.1.1.1" "8.8.8.8" ];
};
```

## Core Services

### Docker

```nix
virtualisation.docker = {
  enable = true;
  enableOnBoot = true;
  autoPrune = {
    enable = true;
    dates = "weekly";
  };
};

# Docker Compose for service stacks
environment.systemPackages = [ pkgs.docker-compose ];
```

### Hashicorp Vault

```nix
services.vault = {
  enable = true;
  address = "0.0.0.0:8200";
  storageBackend = "file";
  storagePath = "/var/lib/vault";
  # TLS configuration
  tlsCertFile = "/var/lib/vault/cert.pem";
  tlsKeyFile = "/var/lib/vault/key.pem";
};
```

### Backup Services

```nix
# Restic backup to Backblaze B2
services.restic.backups.daily = {
  initialize = true;
  repository = "b2:bucket-name:/backups";
  passwordFile = "/run/secrets/restic-password";
  environmentFile = "/run/secrets/b2-credentials";
  paths = [
    "/home"
    "/var/lib"
    "/etc/nixos"
  ];
  timerConfig = {
    OnCalendar = "daily";
    Persistent = true;
  };
};
```

### Monitoring

```nix
# Prometheus
services.prometheus = {
  enable = true;
  port = 9090;
  scrapeConfigs = [
    {
      job_name = "node";
      static_configs = [{
        targets = [ "localhost:9100" ];
      }];
    }
  ];
};

# Node exporter
services.prometheus.exporters.node = {
  enable = true;
  port = 9100;
};

# Grafana
services.grafana = {
  enable = true;
  settings.server = {
    http_addr = "0.0.0.0";
    http_port = 3000;
  };
};
```

## Wireguard VPN Server

### Full Tunnel Configuration

Route all device traffic through this server:

```nix
networking.wireguard.interfaces.wg0 = {
  ips = [ "10.100.0.1/24" ];
  listenPort = 51820;
  privateKeyFile = "/run/secrets/wireguard-server-key";

  # Enable IP forwarding
  postSetup = ''
    ${pkgs.iptables}/bin/iptables -t nat -A POSTROUTING -s 10.100.0.0/24 -o eth0 -j MASQUERADE
  '';
  postShutdown = ''
    ${pkgs.iptables}/bin/iptables -t nat -D POSTROUTING -s 10.100.0.0/24 -o eth0 -j MASQUERADE
  '';

  peers = [
    {
      # Laptop
      publicKey = "LAPTOP_PUBLIC_KEY";
      allowedIPs = [ "10.100.0.2/32" ];
    }
    {
      # Phone
      publicKey = "PHONE_PUBLIC_KEY";
      allowedIPs = [ "10.100.0.3/32" ];
    }
    {
      # Tablet
      publicKey = "TABLET_PUBLIC_KEY";
      allowedIPs = [ "10.100.0.4/32" ];
    }
  ];
};

# Enable IP forwarding
boot.kernel.sysctl = {
  "net.ipv4.ip_forward" = 1;
  "net.ipv6.conf.all.forwarding" = 1;
};
```

## Firewall

```nix
networking.firewall = {
  enable = true;

  allowedTCPPorts = [
    22    # SSH
    80    # HTTP (for ACME)
    443   # HTTPS
    8200  # Vault
    3000  # Grafana
    9090  # Prometheus
  ];

  allowedUDPPorts = [
    51820  # Wireguard
  ];

  # Allow forwarding for VPN
  extraCommands = ''
    iptables -A FORWARD -i wg0 -j ACCEPT
    iptables -A FORWARD -o wg0 -j ACCEPT
  '';
};
```

## DNS Server (Optional)

### Pi-hole Alternative

```nix
services.adguardhome = {
  enable = true;
  port = 3000;
  settings = {
    dns = {
      bind_hosts = [ "0.0.0.0" ];
      port = 53;
      upstream_dns = [ "1.1.1.1" "8.8.8.8" ];
    };
  };
};
```

## Profile Options

### Minimal Profile

- Basic server setup
- SSH access
- Docker
- Firewall

### Full Profile

- Everything in minimal
- Wireguard VPN server
- Hashicorp Vault
- Backup services (Restic + Backblaze)
- Monitoring (Prometheus + Grafana)
- DNS filtering (AdGuard Home)

## Security

### SSH Hardening

```nix
services.openssh = {
  enable = true;
  settings = {
    PasswordAuthentication = false;
    PermitRootLogin = "no";
    KbdInteractiveAuthentication = false;
  };
};
```

### Fail2ban

```nix
services.fail2ban = {
  enable = true;
  maxretry = 3;
  bantime = "1h";
};
```
