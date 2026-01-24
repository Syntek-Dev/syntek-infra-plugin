# Mullvad Wireguard Client Configuration

Outbound VPN configuration using Mullvad for privacy protection.

## Use Case

Route internet traffic through Mullvad VPN for privacy when on untrusted
networks.

## Prerequisites

1. Mullvad account with Wireguard configuration
2. Wireguard private key stored in Vault or file
3. Mullvad server public key and endpoint

## Getting Mullvad Configuration

1. Log in to Mullvad account
2. Go to Wireguard configuration
3. Generate or download configuration
4. Note: private key, address, server public key, endpoint

## NixOS Configuration

```nix
{ config, pkgs, ... }:

{
  # Enable Wireguard
  networking.wireguard.interfaces = {
    mullvad = {
      # Your Mullvad address (from their config)
      ips = [ "10.66.123.45/32" "fc00:bbbb:bbbb:bb01::1234/128" ];

      # Private key from file (never hardcode!)
      privateKeyFile = "/run/secrets/mullvad-private-key";

      # DNS through Mullvad
      dns = [ "10.64.0.1" ];

      peers = [
        {
          # Mullvad server public key
          publicKey = "MULLVAD_SERVER_PUBLIC_KEY_HERE";

          # Route all traffic through VPN
          allowedIPs = [ "0.0.0.0/0" "::/0" ];

          # Mullvad server endpoint
          endpoint = "MULLVAD_SERVER_HOSTNAME:51820";

          # Keep connection alive through NAT
          persistentKeepalive = 25;
        }
      ];

      # Kill switch - block traffic if VPN disconnects
      postSetup = ''
        # Mark VPN traffic
        ${pkgs.iptables}/bin/iptables -t mangle -A OUTPUT -o mullvad -j MARK --set-mark 51820

        # Block non-VPN traffic (kill switch)
        ${pkgs.iptables}/bin/iptables -I OUTPUT ! -o mullvad \
          -m mark ! --mark 51820 \
          -m addrtype ! --dst-type LOCAL \
          -j REJECT

        # IPv6 kill switch
        ${pkgs.iptables}/bin/ip6tables -I OUTPUT ! -o mullvad \
          -m mark ! --mark 51820 \
          -m addrtype ! --dst-type LOCAL \
          -j REJECT
      '';

      postShutdown = ''
        # Remove kill switch rules
        ${pkgs.iptables}/bin/iptables -D OUTPUT ! -o mullvad \
          -m mark ! --mark 51820 \
          -m addrtype ! --dst-type LOCAL \
          -j REJECT || true

        ${pkgs.iptables}/bin/ip6tables -D OUTPUT ! -o mullvad \
          -m mark ! --mark 51820 \
          -m addrtype ! --dst-type LOCAL \
          -j REJECT || true
      '';
    };
  };

  # Firewall configuration
  networking.firewall = {
    allowedUDPPorts = [ 51820 ];
    trustedInterfaces = [ "mullvad" ];
  };
}
```

## With Split Tunneling

Allow certain traffic to bypass VPN:

```nix
{ config, pkgs, ... }:

{
  networking.wireguard.interfaces.mullvad = {
    ips = [ "10.66.123.45/32" ];
    privateKeyFile = "/run/secrets/mullvad-private-key";

    peers = [{
      publicKey = "MULLVAD_SERVER_PUBLIC_KEY";
      # Route most traffic through VPN, but exclude local network
      allowedIPs = [
        "0.0.0.0/1"      # First half of IPv4
        "128.0.0.0/1"    # Second half of IPv4
        # This effectively routes all internet traffic but allows
        # more specific routes (like local network) to take precedence
      ];
      endpoint = "SERVER:51820";
      persistentKeepalive = 25;
    }];

    # Allow LAN access while on VPN
    postSetup = ''
      # Allow local network traffic
      ${pkgs.iproute2}/bin/ip route add 192.168.1.0/24 dev eth0 || true
    '';
  };
}
```

## Systemd Service for Auto-Start

```nix
{
  # Start VPN on boot
  systemd.services."wireguard-mullvad".wantedBy = [ "multi-user.target" ];

  # Or, start VPN when NetworkManager connects
  # networking.networkmanager.dispatcherScripts = [{
  #   source = pkgs.writeText "vpn-up" ''
  #     if [ "$2" = "up" ]; then
  #       systemctl start wireguard-mullvad
  #     fi
  #   '';
  #   type = "basic";
  # }];
}
```

## Multiple Servers (Rotation)

For server rotation, configure multiple peers and switch between them:

```nix
{ config, pkgs, ... }:

let
  mullvadServers = {
    sweden = {
      publicKey = "SWEDEN_PUBLIC_KEY";
      endpoint = "se-sto-wg-001.relays.mullvad.net:51820";
    };
    germany = {
      publicKey = "GERMANY_PUBLIC_KEY";
      endpoint = "de-fra-wg-001.relays.mullvad.net:51820";
    };
    uk = {
      publicKey = "UK_PUBLIC_KEY";
      endpoint = "gb-lon-wg-001.relays.mullvad.net:51820";
    };
  };

  # Select server (can be changed via NixOS option)
  currentServer = mullvadServers.uk;
in
{
  networking.wireguard.interfaces.mullvad = {
    ips = [ "10.66.123.45/32" ];
    privateKeyFile = "/run/secrets/mullvad-private-key";

    peers = [{
      publicKey = currentServer.publicKey;
      allowedIPs = [ "0.0.0.0/0" "::/0" ];
      endpoint = currentServer.endpoint;
      persistentKeepalive = 25;
    }];
  };
}
```

## Verifying VPN is Working

```bash
# Check Wireguard status
sudo wg show mullvad

# Verify IP address
curl https://am.i.mullvad.net/json

# Check for DNS leaks
curl https://am.i.mullvad.net/dns
```
