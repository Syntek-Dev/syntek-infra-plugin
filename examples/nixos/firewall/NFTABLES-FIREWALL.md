# nftables Firewall Configuration

Modern firewall configuration using nftables on NixOS.

## Use Case

nftables is the successor to iptables, offering:

- Cleaner syntax
- Better performance
- Atomic rule updates
- Combined IPv4/IPv6 handling

## Basic nftables Setup

### configuration.nix

```nix
{ config, pkgs, ... }:

{
  # Disable iptables-based firewall
  networking.firewall.enable = false;

  # Enable nftables
  networking.nftables = {
    enable = true;

    ruleset = ''
      table inet filter {
        chain input {
          type filter hook input priority filter; policy drop;

          # Allow established/related connections
          ct state established,related accept

          # Allow loopback
          iif "lo" accept

          # Allow ICMP (ping)
          ip protocol icmp accept
          ip6 nexthdr icmpv6 accept

          # Allow SSH
          tcp dport 22 accept

          # Allow HTTP/HTTPS
          tcp dport { 80, 443 } accept

          # Allow Wireguard
          udp dport 51820 accept

          # Log and drop everything else
          log prefix "nftables drop: " drop
        }

        chain forward {
          type filter hook forward priority filter; policy drop;

          # Allow established/related
          ct state established,related accept

          # Allow forwarding from VPN
          iifname "wg0" accept
          oifname "wg0" accept
        }

        chain output {
          type filter hook output priority filter; policy accept;
        }
      }
    '';
  };
}
```

## Server Firewall with Rate Limiting

```nix
{ config, pkgs, ... }:

{
  networking.nftables = {
    enable = true;

    ruleset = ''
      table inet filter {
        # Rate limit sets
        set ssh_meter {
          type ipv4_addr
          flags dynamic
          timeout 1m
        }

        set http_meter {
          type ipv4_addr
          flags dynamic
          timeout 10s
        }

        chain input {
          type filter hook input priority filter; policy drop;

          # Connection tracking
          ct state invalid drop
          ct state established,related accept

          # Loopback
          iif "lo" accept

          # ICMP with rate limit
          ip protocol icmp limit rate 10/second accept
          ip6 nexthdr icmpv6 limit rate 10/second accept

          # SSH with rate limiting (max 5 new connections per minute per IP)
          tcp dport 22 ct state new \
            add @ssh_meter { ip saddr limit rate 5/minute } accept

          # HTTP/HTTPS with rate limiting
          tcp dport { 80, 443 } ct state new \
            add @http_meter { ip saddr limit rate 100/second } accept

          # Wireguard
          udp dport 51820 accept

          # Drop and log
          counter log prefix "nft-drop: " drop
        }

        chain forward {
          type filter hook forward priority filter; policy drop;
          ct state established,related accept
        }

        chain output {
          type filter hook output priority filter; policy accept;
        }
      }
    '';
  };
}
```

## Router/Gateway Firewall with NAT

```nix
{ config, pkgs, ... }:

{
  # Enable IP forwarding
  boot.kernel.sysctl = {
    "net.ipv4.ip_forward" = 1;
    "net.ipv6.conf.all.forwarding" = 1;
  };

  networking.nftables = {
    enable = true;

    ruleset = ''
      table inet filter {
        chain input {
          type filter hook input priority filter; policy drop;

          ct state established,related accept
          iif "lo" accept

          # Allow from LAN
          iifname "br-lan" accept

          # Allow from VPN
          iifname "wg0" accept

          # ICMP
          ip protocol icmp accept
          ip6 nexthdr icmpv6 accept

          # SSH from LAN only
          iifname "br-lan" tcp dport 22 accept

          # DNS from LAN (if running local DNS)
          iifname "br-lan" udp dport 53 accept
          iifname "br-lan" tcp dport 53 accept

          # DHCP
          udp dport 67 accept

          # Wireguard from WAN
          iifname "eth0" udp dport 51820 accept

          log prefix "input-drop: " drop
        }

        chain forward {
          type filter hook forward priority filter; policy drop;

          ct state established,related accept

          # Allow LAN to WAN
          iifname "br-lan" oifname "eth0" accept

          # Allow LAN to VPN
          iifname "br-lan" oifname "wg0" accept

          # Allow VPN to LAN
          iifname "wg0" oifname "br-lan" accept

          # Allow VPN to WAN
          iifname "wg0" oifname "eth0" accept

          log prefix "forward-drop: " drop
        }

        chain output {
          type filter hook output priority filter; policy accept;
        }
      }

      table inet nat {
        chain prerouting {
          type nat hook prerouting priority dstnat;

          # Port forwarding example: forward port 8080 to internal server
          # iifname "eth0" tcp dport 8080 dnat to 192.168.1.100:80
        }

        chain postrouting {
          type nat hook postrouting priority srcnat;

          # Masquerade traffic going to WAN
          oifname "eth0" masquerade

          # Masquerade VPN traffic if routing through this host
          # oifname "wg0" masquerade
        }
      }
    '';
  };
}
```

## Zone-Based Firewall

For complex networks with multiple security zones:

```nix
{ config, pkgs, ... }:

{
  networking.nftables = {
    enable = true;

    ruleset = ''
      # Define interface sets for zones
      define WAN_IFACE = eth0
      define LAN_IFACE = br-lan
      define DMZ_IFACE = br-dmz
      define VPN_IFACE = wg0

      define LAN_NET = 192.168.1.0/24
      define DMZ_NET = 192.168.10.0/24
      define VPN_NET = 10.100.0.0/24

      table inet filter {
        chain input {
          type filter hook input priority filter; policy drop;

          ct state established,related accept
          iif "lo" accept

          # Zone: WAN (untrusted)
          iifname $WAN_IFACE jump input_wan

          # Zone: LAN (trusted)
          iifname $LAN_IFACE jump input_lan

          # Zone: DMZ (semi-trusted)
          iifname $DMZ_IFACE jump input_dmz

          # Zone: VPN (trusted)
          iifname $VPN_IFACE jump input_vpn
        }

        chain input_wan {
          # Only allow Wireguard from WAN
          udp dport 51820 accept

          # ICMP for path MTU discovery
          icmp type { echo-request, destination-unreachable, time-exceeded } accept

          return
        }

        chain input_lan {
          # Trust LAN
          accept
        }

        chain input_dmz {
          # Only allow specific traffic from DMZ
          tcp dport 22 accept  # SSH
          return
        }

        chain input_vpn {
          # Trust VPN
          accept
        }

        chain forward {
          type filter hook forward priority filter; policy drop;

          ct state established,related accept

          # LAN -> anywhere
          iifname $LAN_IFACE accept

          # VPN -> LAN, WAN
          iifname $VPN_IFACE oifname { $LAN_IFACE, $WAN_IFACE } accept

          # DMZ -> WAN only
          iifname $DMZ_IFACE oifname $WAN_IFACE accept

          # WAN -> DMZ (for public services)
          iifname $WAN_IFACE oifname $DMZ_IFACE tcp dport { 80, 443 } accept
        }

        chain output {
          type filter hook output priority filter; policy accept;
        }
      }

      table inet nat {
        chain postrouting {
          type nat hook postrouting priority srcnat;
          oifname $WAN_IFACE masquerade
        }
      }
    '';
  };
}
```

## Combining with NixOS Firewall Options

You can use nftables while still using some NixOS firewall helpers:

```nix
{ config, pkgs, ... }:

{
  networking.nftables.enable = true;

  # Use NixOS firewall options (converted to nftables)
  networking.firewall = {
    enable = true;

    allowedTCPPorts = [ 22 80 443 ];
    allowedUDPPorts = [ 51820 ];

    # Per-interface rules
    interfaces = {
      "wg0" = {
        allowedTCPPorts = [ 8080 ];
      };
    };

    # Additional nftables rules
    extraInputRules = ''
      iifname "br-lan" accept
    '';
  };
}
```

## Debugging nftables

```bash
# List all rules
sudo nft list ruleset

# List specific table
sudo nft list table inet filter

# Monitor rule matches in real-time
sudo nft monitor

# Check counter values
sudo nft list chain inet filter input

# Flush all rules (careful!)
# sudo nft flush ruleset
```
