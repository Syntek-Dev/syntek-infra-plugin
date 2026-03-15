# Custom NixOS Module Pattern

**Last Updated:** 15/03/2026
**Version:** 2.0.0
**Maintained By:** Development Team
**Language:** British English (en_GB)
**Timezone:** Europe/London

---

Creating reusable NixOS modules with options.

## Use Case

Encapsulate configuration into reusable modules with configurable options.
Essential for:

- Sharing configuration across multiple hosts
- Creating toggleable features
- Abstracting complex service setups

## Basic Module Structure

### modules/my-service.nix

```nix
{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.services.myService;
in
{
  # Define options for this module
  options.services.myService = {
    enable = mkEnableOption "my custom service";

    port = mkOption {
      type = types.port;
      default = 8080;
      description = "Port to listen on";
    };

    dataDir = mkOption {
      type = types.path;
      default = "/var/lib/my-service";
      description = "Directory for service data";
    };

    user = mkOption {
      type = types.str;
      default = "myservice";
      description = "User to run the service as";
    };

    openFirewall = mkOption {
      type = types.bool;
      default = false;
      description = "Open firewall port for the service";
    };

    settings = mkOption {
      type = types.attrs;
      default = {};
      description = "Additional settings passed to the service";
    };
  };

  # Implementation when enabled
  config = mkIf cfg.enable {
    # Create service user
    users.users.${cfg.user} = {
      isSystemUser = true;
      group = cfg.user;
      home = cfg.dataDir;
      createHome = true;
    };

    users.groups.${cfg.user} = {};

    # Create data directory
    systemd.tmpfiles.rules = [
      "d ${cfg.dataDir} 0750 ${cfg.user} ${cfg.user} -"
    ];

    # Systemd service
    systemd.services.my-service = {
      description = "My Custom Service";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];

      serviceConfig = {
        Type = "simple";
        User = cfg.user;
        Group = cfg.user;
        ExecStart = "${pkgs.my-package}/bin/my-service --port ${toString cfg.port}";
        Restart = "on-failure";
        RestartSec = 5;

        # Security hardening
        NoNewPrivileges = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        PrivateTmp = true;
        ReadWritePaths = [ cfg.dataDir ];
      };
    };

    # Firewall
    networking.firewall.allowedTCPPorts = mkIf cfg.openFirewall [ cfg.port ];
  };
}
```

## Using the Module

### configuration.nix

```nix
{ config, pkgs, ... }:

{
  imports = [
    ./modules/my-service.nix
  ];

  services.myService = {
    enable = true;
    port = 9000;
    openFirewall = true;
    settings = {
      logLevel = "info";
      maxConnections = 100;
    };
  };
}
```

## Module with Submodules

For complex configurations with nested options:

### modules/vpn-endpoints.nix

```nix
{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.networking.vpnEndpoints;

  # Define the submodule type
  endpointModule = types.submodule {
    options = {
      publicKey = mkOption {
        type = types.str;
        description = "Wireguard public key";
      };

      endpoint = mkOption {
        type = types.nullOr types.str;
        default = null;
        description = "Endpoint address:port";
      };

      allowedIPs = mkOption {
        type = types.listOf types.str;
        default = [ "10.100.0.0/24" ];
        description = "Allowed IP ranges";
      };

      persistentKeepalive = mkOption {
        type = types.nullOr types.int;
        default = 25;
        description = "Keepalive interval in seconds";
      };
    };
  };
in
{
  options.networking.vpnEndpoints = {
    enable = mkEnableOption "VPN endpoint management";

    interface = mkOption {
      type = types.str;
      default = "wg0";
      description = "Wireguard interface name";
    };

    address = mkOption {
      type = types.str;
      example = "10.100.0.1/24";
      description = "This host's VPN address";
    };

    privateKeyFile = mkOption {
      type = types.path;
      description = "Path to private key file";
    };

    peers = mkOption {
      type = types.attrsOf endpointModule;
      default = {};
      description = "VPN peer configurations";
    };
  };

  config = mkIf cfg.enable {
    networking.wireguard.interfaces.${cfg.interface} = {
      ips = [ cfg.address ];
      privateKeyFile = cfg.privateKeyFile;

      peers = mapAttrsToList (name: peer: {
        inherit (peer) publicKey allowedIPs;
        endpoint = peer.endpoint;
        persistentKeepalive = peer.persistentKeepalive;
      }) cfg.peers;
    };
  };
}
```

### Using Submodules

```nix
{
  networking.vpnEndpoints = {
    enable = true;
    interface = "wg0";
    address = "10.100.0.1/24";
    privateKeyFile = "/run/secrets/wireguard-key";

    peers = {
      laptop = {
        publicKey = "LAPTOP_PUBLIC_KEY";
        allowedIPs = [ "10.100.0.2/32" ];
      };

      phone = {
        publicKey = "PHONE_PUBLIC_KEY";
        allowedIPs = [ "10.100.0.3/32" ];
      };

      remote-server = {
        publicKey = "SERVER_PUBLIC_KEY";
        endpoint = "server.example.com:51820";
        allowedIPs = [ "10.100.0.10/32" "192.168.10.0/24" ];
      };
    };
  };
}
```

## Module with Assertions

Add validation to catch configuration errors:

```nix
{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.services.myService;
in
{
  options.services.myService = {
    # ... options ...
  };

  config = mkIf cfg.enable {
    assertions = [
      {
        assertion = cfg.port > 1024 || config.services.myService.user == "root";
        message = "Ports below 1024 require root user";
      }
      {
        assertion = cfg.settings ? logLevel -> elem cfg.settings.logLevel [ "debug" "info" "warn" "error" ];
        message = "logLevel must be one of: debug, info, warn, error";
      }
    ];

    warnings = optional (cfg.port == 8080)
      "Using default port 8080 for myService - consider changing for production";

    # ... rest of config ...
  };
}
```

## Flake Module Pattern

For modules distributed via flakes:

### flake.nix (module provider)

```nix
{
  outputs = { self, nixpkgs }: {
    nixosModules.default = import ./modules/my-service.nix;
    nixosModules.my-service = import ./modules/my-service.nix;
  };
}
```

### Consumer flake.nix

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    my-modules.url = "github:user/my-modules";
  };

  outputs = { self, nixpkgs, my-modules }: {
    nixosConfigurations.my-host = nixpkgs.lib.nixosSystem {
      modules = [
        my-modules.nixosModules.my-service
        ./configuration.nix
      ];
    };
  };
}
```
