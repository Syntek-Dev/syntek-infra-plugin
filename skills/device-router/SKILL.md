# Device: DIY Router

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

This skill provides guidance for configuring NixOS as a DIY router.

## Router Role

The DIY router acts as:

- Network gateway and firewall
- DHCP and DNS server
- Wireguard VPN endpoint
- Smart home controller hub
- Traffic shaping and QoS

## Hardware Considerations

### Network Interfaces

Typical setup requires at least 2 NICs:

- `wan` - Connection to ISP
- `lan` - Internal network (may be bridged with multiple ports)

```nix
networking = {
  useDHCP = false;

  # WAN interface (DHCP from ISP)
  interfaces.enp1s0 = {
    useDHCP = true;
  };

  # LAN interface (static)
  interfaces.enp2s0 = {
    ipv4.addresses = [{
      address = "192.168.1.1";
      prefixLength = 24;
    }];
  };
};
```

### Bridge for Multiple LAN Ports

```nix
networking.bridges.br-lan = {
  interfaces = [ "enp2s0" "enp3s0" "enp4s0" ];
};

networking.interfaces.br-lan = {
  ipv4.addresses = [{
    address = "192.168.1.1";
    prefixLength = 24;
  }];
};
```

## Core Services

### NAT and Forwarding

```nix
networking.nat = {
  enable = true;
  internalInterfaces = [ "br-lan" "wg0" ];
  externalInterface = "enp1s0";
};

boot.kernel.sysctl = {
  "net.ipv4.ip_forward" = 1;
  "net.ipv6.conf.all.forwarding" = 1;
};
```

### DHCP Server

```nix
services.dnsmasq = {
  enable = true;
  settings = {
    interface = "br-lan";
    dhcp-range = [ "192.168.1.100,192.168.1.200,24h" ];
    dhcp-option = [
      "option:router,192.168.1.1"
      "option:dns-server,192.168.1.1"
    ];
    # Static leases
    dhcp-host = [
      "aa:bb:cc:dd:ee:ff,server,192.168.1.10"
      "11:22:33:44:55:66,printer,192.168.1.20"
    ];
  };
};
```

### DNS Server

```nix
services.dnsmasq.settings = {
  # Upstream DNS
  server = [ "1.1.1.1" "8.8.8.8" ];

  # Local domain
  local = "/home.lan/";
  domain = "home.lan";

  # Block ads (optional)
  conf-file = "/etc/dnsmasq.d/blocklist.conf";
};
```

### Firewall

```nix
networking.firewall = {
  enable = true;

  # Allow from LAN
  interfaces.br-lan = {
    allowedTCPPorts = [ 22 53 80 443 ];
    allowedUDPPorts = [ 53 67 68 51820 ];
  };

  # WAN - minimal exposure
  interfaces.enp1s0 = {
    allowedUDPPorts = [ 51820 ];  # Wireguard only
  };
};

# Advanced firewall rules with nftables
networking.nftables = {
  enable = true;
  ruleset = ''
    table inet filter {
      chain input {
        type filter hook input priority 0; policy drop;

        # Allow established
        ct state established,related accept

        # Allow loopback
        iif lo accept

        # Allow LAN
        iifname "br-lan" accept

        # Allow Wireguard from WAN
        iifname "enp1s0" udp dport 51820 accept

        # ICMP
        ip protocol icmp accept
      }

      chain forward {
        type filter hook forward priority 0; policy drop;

        # Allow established
        ct state established,related accept

        # Allow LAN to WAN
        iifname "br-lan" oifname "enp1s0" accept

        # Allow VPN to LAN
        iifname "wg0" oifname "br-lan" accept
      }
    }
  '';
};
```

## Wireguard VPN

### Road Warrior Access

Allow remote devices to access home network:

```nix
networking.wireguard.interfaces.wg0 = {
  ips = [ "10.100.0.1/24" ];
  listenPort = 51820;
  privateKeyFile = "/run/secrets/wireguard-router-key";

  peers = [
    {
      publicKey = "LAPTOP_PUBLIC_KEY";
      allowedIPs = [ "10.100.0.2/32" ];
    }
    {
      publicKey = "PHONE_PUBLIC_KEY";
      allowedIPs = [ "10.100.0.3/32" ];
    }
  ];
};
```

## Smart Home Hub

### Zigbee/Z-Wave Support

```nix
# For Zigbee dongle
services.zigbee2mqtt = {
  enable = true;
  settings = {
    homeassistant = true;
    serial.port = "/dev/ttyUSB0";
    mqtt = {
      server = "mqtt://localhost:1883";
    };
  };
};

# MQTT broker
services.mosquitto = {
  enable = true;
  listeners = [{
    port = 1883;
    users = {
      zigbee2mqtt = {
        acl = [ "readwrite #" ];
        passwordFile = "/run/secrets/mqtt-password";
      };
    };
  }];
};
```

### Home Assistant

```nix
services.home-assistant = {
  enable = true;
  config = {
    homeassistant = {
      name = "Home";
      unit_system = "metric";
      time_zone = "Europe/London";
    };
  };
};
```

## Traffic Shaping (QoS)

```nix
# Simple QoS with tc
networking.extraCommands = ''
  # Clear existing rules
  tc qdisc del dev enp1s0 root 2>/dev/null || true

  # Add root qdisc
  tc qdisc add dev enp1s0 root handle 1: htb default 30

  # Main class (total bandwidth)
  tc class add dev enp1s0 parent 1: classid 1:1 htb rate 100mbit

  # High priority (VoIP, gaming)
  tc class add dev enp1s0 parent 1:1 classid 1:10 htb rate 30mbit ceil 100mbit prio 1

  # Normal priority
  tc class add dev enp1s0 parent 1:1 classid 1:20 htb rate 50mbit ceil 100mbit prio 2

  # Low priority (bulk downloads)
  tc class add dev enp1s0 parent 1:1 classid 1:30 htb rate 20mbit ceil 100mbit prio 3
'';
```

## Profile Options

### Minimal Profile

- Basic routing and NAT
- DHCP server
- DNS forwarding
- Firewall

### Full Profile

- Everything in minimal
- Wireguard VPN server
- DNS filtering
- Smart home hub (Zigbee2MQTT, MQTT, Home Assistant)
- Traffic shaping
- Monitoring

## Security

### Disable Unnecessary Services

```nix
# Minimal attack surface
services.avahi.enable = false;
services.printing.enable = false;
```

### Regular Updates

```nix
system.autoUpgrade = {
  enable = true;
  allowReboot = false;  # Manual reboot for routers
  dates = "04:00";
};
```
