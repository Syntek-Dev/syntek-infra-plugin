# Network Segmentation with VLANs

VLAN configuration for network isolation on NixOS.

## Use Case

Network segmentation provides:

- Security isolation between network zones
- Separate IoT devices from trusted devices
- Guest network isolation
- Server/DMZ separation
- Traffic management and QoS

## Basic VLAN Configuration

### Single Interface with Multiple VLANs

```nix
{ config, pkgs, ... }:

{
  # Enable VLAN support
  networking.vlans = {
    # Management VLAN (ID 10)
    vlan10 = {
      id = 10;
      interface = "eth0";
    };

    # Trusted LAN VLAN (ID 20)
    vlan20 = {
      id = 20;
      interface = "eth0";
    };

    # IoT VLAN (ID 30)
    vlan30 = {
      id = 30;
      interface = "eth0";
    };

    # Guest VLAN (ID 40)
    vlan40 = {
      id = 40;
      interface = "eth0";
    };
  };

  # Configure VLAN interfaces
  networking.interfaces = {
    # Trunk port (no IP on base interface)
    eth0.useDHCP = false;

    # Management network
    vlan10 = {
      ipv4.addresses = [{
        address = "10.10.0.1";
        prefixLength = 24;
      }];
    };

    # Trusted LAN
    vlan20 = {
      ipv4.addresses = [{
        address = "10.20.0.1";
        prefixLength = 24;
      }];
    };

    # IoT network
    vlan30 = {
      ipv4.addresses = [{
        address = "10.30.0.1";
        prefixLength = 24;
      }];
    };

    # Guest network
    vlan40 = {
      ipv4.addresses = [{
        address = "10.40.0.1";
        prefixLength = 24;
      }];
    };
  };
}
```

## Router with VLAN Segmentation

Full router configuration with inter-VLAN routing and firewall:

```nix
{ config, pkgs, ... }:

{
  # Enable IP forwarding
  boot.kernel.sysctl = {
    "net.ipv4.ip_forward" = 1;
    "net.ipv6.conf.all.forwarding" = 1;
  };

  # VLAN definitions
  networking.vlans = {
    # Trusted LAN
    lan = { id = 20; interface = "eth0"; };
    # IoT devices
    iot = { id = 30; interface = "eth0"; };
    # Guest network
    guest = { id = 40; interface = "eth0"; };
    # Server/DMZ
    dmz = { id = 50; interface = "eth0"; };
  };

  networking.interfaces = {
    # WAN interface
    eth1 = {
      useDHCP = true;
    };

    # Trunk interface (no IP)
    eth0.useDHCP = false;

    # VLAN interfaces
    lan = {
      ipv4.addresses = [{ address = "10.20.0.1"; prefixLength = 24; }];
    };
    iot = {
      ipv4.addresses = [{ address = "10.30.0.1"; prefixLength = 24; }];
    };
    guest = {
      ipv4.addresses = [{ address = "10.40.0.1"; prefixLength = 24; }];
    };
    dmz = {
      ipv4.addresses = [{ address = "10.50.0.1"; prefixLength = 24; }];
    };
  };

  # DHCP server for each VLAN
  services.kea.dhcp4 = {
    enable = true;
    settings = {
      interfaces-config = {
        interfaces = [ "lan" "iot" "guest" "dmz" ];
      };

      subnet4 = [
        {
          id = 1;
          subnet = "10.20.0.0/24";
          pools = [{ pool = "10.20.0.100 - 10.20.0.200"; }];
          option-data = [
            { name = "routers"; data = "10.20.0.1"; }
            { name = "domain-name-servers"; data = "10.20.0.1"; }
          ];
        }
        {
          id = 2;
          subnet = "10.30.0.0/24";
          pools = [{ pool = "10.30.0.100 - 10.30.0.200"; }];
          option-data = [
            { name = "routers"; data = "10.30.0.1"; }
            { name = "domain-name-servers"; data = "10.30.0.1"; }
          ];
        }
        {
          id = 3;
          subnet = "10.40.0.0/24";
          pools = [{ pool = "10.40.0.100 - 10.40.0.200"; }];
          option-data = [
            { name = "routers"; data = "10.40.0.1"; }
            { name = "domain-name-servers"; data = "1.1.1.1, 8.8.8.8"; }
          ];
        }
        {
          id = 4;
          subnet = "10.50.0.0/24";
          pools = [{ pool = "10.50.0.100 - 10.50.0.200"; }];
          option-data = [
            { name = "routers"; data = "10.50.0.1"; }
            { name = "domain-name-servers"; data = "10.50.0.1"; }
          ];
        }
      ];
    };
  };

  # nftables firewall with VLAN rules
  networking.nftables = {
    enable = true;
    ruleset = ''
      table inet filter {
        chain input {
          type filter hook input priority filter; policy drop;

          ct state established,related accept
          iif "lo" accept

          # Allow from trusted LAN
          iifname "lan" accept

          # Allow DHCP from all VLANs
          udp dport 67 accept

          # Allow DNS from all internal VLANs
          iifname { "lan", "iot", "dmz" } udp dport 53 accept
          iifname { "lan", "iot", "dmz" } tcp dport 53 accept

          # Guest only gets DHCP and DNS
          iifname "guest" drop
        }

        chain forward {
          type filter hook forward priority filter; policy drop;

          ct state established,related accept

          # Trusted LAN can access everything
          iifname "lan" accept

          # IoT can only access internet, not other VLANs
          iifname "iot" oifname "eth1" accept

          # Guest can only access internet
          iifname "guest" oifname "eth1" accept

          # DMZ can access internet
          iifname "dmz" oifname "eth1" accept

          # Allow WAN to DMZ for public services
          iifname "eth1" oifname "dmz" tcp dport { 80, 443 } accept
        }

        chain output {
          type filter hook output priority filter; policy accept;
        }
      }

      table inet nat {
        chain postrouting {
          type nat hook postrouting priority srcnat;
          oifname "eth1" masquerade
        }

        chain prerouting {
          type nat hook prerouting priority dstnat;
          # Port forward to DMZ web server
          iifname "eth1" tcp dport 80 dnat to 10.50.0.10
          iifname "eth1" tcp dport 443 dnat to 10.50.0.10
        }
      }
    '';
  };
}
```

## Bridge with VLANs

For virtual machines or containers with VLAN access:

```nix
{ config, pkgs, ... }:

{
  networking.bridges = {
    br-lan = {
      interfaces = [ "vlan20" ];
    };
    br-dmz = {
      interfaces = [ "vlan50" ];
    };
  };

  networking.vlans = {
    vlan20 = { id = 20; interface = "eth0"; };
    vlan50 = { id = 50; interface = "eth0"; };
  };

  networking.interfaces = {
    eth0.useDHCP = false;

    br-lan = {
      ipv4.addresses = [{ address = "10.20.0.1"; prefixLength = 24; }];
    };

    br-dmz = {
      ipv4.addresses = [{ address = "10.50.0.1"; prefixLength = 24; }];
    };
  };

  # VMs can attach to these bridges
  # virtualisation.libvirtd.enable = true;
}
```

## Wireless with VLANs (hostapd)

Separate SSIDs on different VLANs:

```nix
{ config, pkgs, ... }:

{
  networking.vlans = {
    vlan20 = { id = 20; interface = "eth0"; };  # Trusted
    vlan40 = { id = 40; interface = "eth0"; };  # Guest
  };

  # Bridge for wireless interfaces
  networking.bridges = {
    br-trusted = { interfaces = [ "vlan20" ]; };
    br-guest = { interfaces = [ "vlan40" ]; };
  };

  services.hostapd = {
    enable = true;
    radios.wlan0 = {
      band = "2g";
      channel = 6;

      networks = {
        # Trusted network
        wlan0 = {
          ssid = "Home-Trusted";
          authentication = {
            mode = "wpa3-sae";
            saePasswords = [{ password = "WIFI_PASSWORD"; }];
          };
          bssid = "02:00:00:00:00:01";
          settings.bridge = "br-trusted";
        };

        # Guest network (separate BSSID)
        wlan0-guest = {
          ssid = "Home-Guest";
          authentication = {
            mode = "wpa2-sha256";
            wpaPassword = "GUEST_PASSWORD";
          };
          bssid = "02:00:00:00:00:02";
          settings.bridge = "br-guest";
        };
      };
    };
  };
}
```

## VLAN Tagging for Servers

Server with tagged VLAN access:

```nix
{ config, pkgs, ... }:

{
  # Server connects to multiple VLANs
  networking.vlans = {
    # Management VLAN
    mgmt = { id = 10; interface = "eth0"; };
    # Application VLAN
    app = { id = 60; interface = "eth0"; };
    # Database VLAN
    db = { id = 70; interface = "eth0"; };
    # Storage VLAN
    storage = { id = 80; interface = "eth0"; };
  };

  networking.interfaces = {
    eth0.useDHCP = false;

    mgmt = {
      ipv4.addresses = [{ address = "10.10.0.50"; prefixLength = 24; }];
    };
    app = {
      ipv4.addresses = [{ address = "10.60.0.50"; prefixLength = 24; }];
    };
    db = {
      ipv4.addresses = [{ address = "10.70.0.50"; prefixLength = 24; }];
    };
    storage = {
      ipv4.addresses = [{ address = "10.80.0.50"; prefixLength = 24; }];
    };
  };

  # Default gateway via management VLAN
  networking.defaultGateway = {
    address = "10.10.0.1";
    interface = "mgmt";
  };

  # Bind services to specific VLANs
  services.postgresql = {
    enable = true;
    settings = {
      listen_addresses = "10.70.0.50";  # Only on DB VLAN
    };
  };
}
```

## Debugging VLANs

```bash
# List VLAN interfaces
ip -d link show type vlan

# Show VLAN details
cat /proc/net/vlan/config

# Check traffic on VLAN
sudo tcpdump -i vlan20 -n

# Verify tagging with tcpdump on trunk
sudo tcpdump -i eth0 -e -n vlan

# Test connectivity between VLANs
ping -I vlan20 10.30.0.1
```
