# Full Tunnel Wireguard Server Configuration

Route all client device traffic through a home/business server.

## Use Case

Route all traffic from laptops, phones, and tablets through your home server
for:

- Consistent firewall rules across all devices
- DNS filtering (Pi-hole/AdGuard)
- Accessing home network from anywhere
- Privacy from local network when travelling

## Network Topology

```
[Laptop/Phone/Tablet] → [Internet] → [Home Server] → [Internet]
      10.100.0.x                      10.100.0.1
                                          ↓
                                    [Home Network]
                                    192.168.1.0/24
```

## Server Configuration

```nix
{ config, pkgs, ... }:

{
  # Enable IP forwarding (required for routing)
  boot.kernel.sysctl = {
    "net.ipv4.ip_forward" = 1;
    "net.ipv6.conf.all.forwarding" = 1;
  };

  # Wireguard server
  networking.wireguard.interfaces = {
    wg0 = {
      # Server's VPN IP
      ips = [ "10.100.0.1/24" ];

      # Listen port
      listenPort = 51820;

      # Private key from Vault
      privateKeyFile = "/run/secrets/wireguard-server-key";

      # NAT for client traffic to internet
      postSetup = ''
        # Enable NAT for VPN clients
        ${pkgs.iptables}/bin/iptables -t nat -A POSTROUTING \
          -s 10.100.0.0/24 -o eth0 -j MASQUERADE

        # Allow forwarding
        ${pkgs.iptables}/bin/iptables -A FORWARD -i wg0 -j ACCEPT
        ${pkgs.iptables}/bin/iptables -A FORWARD -o wg0 -j ACCEPT
      '';

      postShutdown = ''
        ${pkgs.iptables}/bin/iptables -t nat -D POSTROUTING \
          -s 10.100.0.0/24 -o eth0 -j MASQUERADE || true
        ${pkgs.iptables}/bin/iptables -D FORWARD -i wg0 -j ACCEPT || true
        ${pkgs.iptables}/bin/iptables -D FORWARD -o wg0 -j ACCEPT || true
      '';

      # Client peers
      peers = [
        {
          # Laptop
          publicKey = "LAPTOP_PUBLIC_KEY_FROM_VAULT";
          allowedIPs = [ "10.100.0.2/32" ];
        }
        {
          # Phone
          publicKey = "PHONE_PUBLIC_KEY_FROM_VAULT";
          allowedIPs = [ "10.100.0.3/32" ];
        }
        {
          # Tablet
          publicKey = "TABLET_PUBLIC_KEY_FROM_VAULT";
          allowedIPs = [ "10.100.0.4/32" ];
        }
        {
          # Work laptop
          publicKey = "WORK_LAPTOP_PUBLIC_KEY_FROM_VAULT";
          allowedIPs = [ "10.100.0.5/32" ];
        }
      ];
    };
  };

  # Firewall
  networking.firewall = {
    enable = true;

    allowedUDPPorts = [ 51820 ];

    # Trust VPN interface
    trustedInterfaces = [ "wg0" ];

    # Additional rules for forwarding
    extraCommands = ''
      # Allow established connections
      iptables -A FORWARD -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT
    '';
  };

  # DNS for VPN clients (optional - use with AdGuard/Pi-hole)
  # services.dnsmasq = {
  #   enable = true;
  #   settings = {
  #     interface = "wg0";
  #     bind-interfaces = true;
  #   };
  # };
}
```

## Client Configuration

### Laptop Client

```nix
{ config, pkgs, ... }:

{
  networking.wireguard.interfaces = {
    wg0 = {
      ips = [ "10.100.0.2/24" ];
      privateKeyFile = "/run/secrets/wireguard-laptop-key";

      # Optional: Use server's DNS
      dns = [ "10.100.0.1" ];

      peers = [{
        publicKey = "SERVER_PUBLIC_KEY";

        # Route ALL traffic through server
        allowedIPs = [ "0.0.0.0/0" "::/0" ];

        # Server endpoint (use DDNS if dynamic IP)
        endpoint = "home.example.com:51820";

        # Keep alive for NAT
        persistentKeepalive = 25;
      }];
    };
  };
}
```

### Phone Client (conf file for import)

Generate this config and create QR code:

```ini
[Interface]
PrivateKey = PHONE_PRIVATE_KEY
Address = 10.100.0.3/24
DNS = 10.100.0.1

[Peer]
PublicKey = SERVER_PUBLIC_KEY
AllowedIPs = 0.0.0.0/0, ::/0
Endpoint = home.example.com:51820
PersistentKeepalive = 25
```

Generate QR code:

```bash
syntek-infra-tool wireguard qr --config phone.conf
```

## Dynamic DNS

If your home IP changes, use Dynamic DNS:

```nix
{
  # Using ddclient for Dynamic DNS
  services.ddclient = {
    enable = true;
    protocol = "cloudflare";
    zone = "example.com";
    domains = [ "home.example.com" ];
    username = "email@example.com";
    passwordFile = "/run/secrets/cloudflare-api-key";
    interval = "5min";
  };
}
```

## Port Forwarding

On your router, forward UDP port 51820 to the server's local IP.

## Adding New Clients

1. Generate key pair:

   ```bash
   syntek-infra-tool wireguard keygen
   ```

2. Store in Vault:

   ```bash
   syntek-infra-tool vault write \
     --path secret/wireguard/devices/new-device \
     --data '{"private_key": "...", "public_key": "..."}'
   ```

3. Add peer to server config with next available IP

4. Deploy server:

   ```bash
   sudo nixos-rebuild switch --flake .#server
   ```

5. Configure client device with its private key and server public key

## Monitoring

```bash
# Server: Check connected clients
sudo wg show wg0

# Server: Watch for connections
sudo wg show wg0 latest-handshakes

# Client: Verify tunnel
curl ifconfig.me  # Should show server's public IP
```
