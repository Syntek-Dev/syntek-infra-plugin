# NixOS DIY Router Configuration

**Device:** [Insert Device Name] **Hostname:** [Insert Hostname] **Stack:**
NixOS + Flakes **Created:** [Insert Date]

## Skill Target

```
Skill Target: device-router
```

## Hardware

- **CPU:** [Specify model]
- **RAM:** [Amount in GB]
- **Storage:** [SSD size]
- **WAN Interface:** [NIC for ISP connection]
- **LAN Interfaces:** [NICs for internal network]

## Profile

Select one:

- [ ] **Minimal** - Basic routing, NAT, DHCP, firewall
- [x] **Full** - Complete router with all features

## Router Role

This router will provide:

- [x] NAT and IP forwarding
- [x] DHCP server
- [x] DNS forwarding
- [x] Firewall
- [x] Wireguard VPN endpoint
- [ ] DNS filtering (AdGuard Home)
- [ ] Traffic shaping (QoS)
- [ ] Smart home hub

## Network Configuration

### WAN (Internet)

- **Interface:** [e.g., enp1s0]
- **Type:** DHCP from ISP

### LAN (Internal)

- **Interface:** [e.g., enp2s0 or br-lan]
- **IP Address:** 192.168.1.1/24
- **DHCP Range:** 192.168.1.100 - 192.168.1.200

### Bridge (if multiple LAN ports)

- **Bridge Name:** br-lan
- **Interfaces:** [e.g., enp2s0, enp3s0, enp4s0]

## DHCP Static Leases

| Device  | MAC Address       | IP Address   |
| ------- | ----------------- | ------------ |
| server  | aa:bb:cc:dd:ee:ff | 192.168.1.10 |
| printer | 11:22:33:44:55:66 | 192.168.1.20 |

## Wireguard VPN

### Road Warrior Access

- **Server IP:** 10.100.0.1/24
- **Listen Port:** 51820

| Client | IP Address |
| ------ | ---------- |
| laptop | 10.100.0.2 |
| phone  | 10.100.0.3 |

## Firewall Rules

### WAN (Incoming)

- Allow: Wireguard (UDP 51820)
- Deny: Everything else

### LAN (Incoming)

- Allow: SSH, DNS, DHCP, HTTP/HTTPS
- Allow: Wireguard

## Smart Home (Optional)

- [ ] Zigbee2MQTT
- [ ] MQTT broker (Mosquitto)
- [ ] Home Assistant

## Placeholders to Replace

When using this template, replace:

- `[Insert Device Name]` - e.g., "PC Engines APU2"
- `[Insert Hostname]` - e.g., "router"
- `[Insert Date]` - e.g., "22/01/2026"
- Interface names
- Static lease entries
- VPN client details
