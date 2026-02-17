# NixOS Mobile Device Configuration

**Device:** [Pinephone / Pinetab] **Hostname:** [Insert Hostname] **Stack:**
NixOS + Flakes + mobile-nixos **Created:** [Insert Date]

## Coding Principles

All code in this project follows Rob Pike's 5 Rules and Linus Torvalds' Coding
Principles. See [coding-principles.md](coding-principles.md) before writing or
reviewing any code.

## Skill Target

```
Skill Target: device-mobile
```

## Hardware

Select device:

- [ ] **Pinephone** - Pine64 smartphone
- [ ] **Pinetab** - Pine64 tablet

### Pinephone Variants

- [ ] Pinephone (original)
- [ ] Pinephone Pro

## Profile

Select one:

- [ ] **Minimal** - Basic phone functionality
- [x] **Full** - Complete mobile device with all features

## Mobile Environment

Select shell:

- [x] **Phosh** - GNOME-based mobile shell (recommended)
- [ ] **Plasma Mobile** - KDE-based mobile shell

## Features

### Communication

- [x] Phone calls (Calls app)
- [x] SMS/MMS (Chatty)
- [x] Email (Geary)

### Connectivity

- [x] WiFi (NetworkManager)
- [x] Mobile data (ModemManager)
- [x] Bluetooth

### Wireguard VPN

- [x] Home access (road warrior)
- [ ] Mullvad (privacy on public WiFi)

## Applications

### Essential

- [x] Web browser (Epiphany)
- [x] Contacts
- [x] Calendar
- [x] File manager
- [x] Camera (Megapixels)

### Additional

- [ ] Maps
- [ ] Music player
- [ ] Flatpak support

## Hardware Features

- [x] Camera
- [x] GPS
- [x] Accelerometer
- [x] Light sensor
- [ ] Fingerprint (if available)

## Wireguard Configuration

### Home Access

```
Server: home.example.com:51820
Client IP: 10.100.0.3/24
Allowed IPs: 10.100.0.0/24, 192.168.1.0/24
```

### Mullvad (Optional)

```
Server: [Mullvad server]
Client IP: [From Mullvad]
Allowed IPs: 0.0.0.0/0, ::/0
```

## Power Management

- **Suspend on:** Power button, screen timeout
- **Wake on:** Incoming calls, alarms

## User Configuration

- **Username:** mobile
- **Initial Password:** changeme (change on first boot)
- **Groups:** wheel, networkmanager, video, audio, dialout

## Known Limitations

Be aware of current limitations:

1. Battery life may be less than Android/iOS
2. Camera quality is improving but not perfect
3. Some apps may not be touch-optimised
4. Hardware support varies by device variant

## Testing Notes

Before deploying to device:

1. Test in QEMU emulation if possible
2. Have SSH access enabled for debugging
3. Keep a backup boot option available

## Placeholders to Replace

When using this template, replace:

- `[Pinephone / Pinetab]` - Select your device
- `[Insert Hostname]` - e.g., "pinephone"
- `[Insert Date]` - e.g., "22/01/2026"
- Wireguard server details
- Select appropriate checkboxes
