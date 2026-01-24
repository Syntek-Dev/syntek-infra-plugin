# systemd Service Deployment

Creating and managing systemd services in NixOS.

## Use Case

Deploy custom applications, scripts, and services with:

- Automatic startup
- Restart on failure
- Security hardening
- Resource limits
- Logging integration

## Basic Service

### Simple Script Service

```nix
{ config, pkgs, ... }:

{
  systemd.services.my-script = {
    description = "My Custom Script";
    wantedBy = [ "multi-user.target" ];
    after = [ "network.target" ];

    serviceConfig = {
      Type = "simple";
      ExecStart = "${pkgs.bash}/bin/bash /opt/scripts/my-script.sh";
      Restart = "on-failure";
      RestartSec = 5;
    };
  };
}
```

### Application Service

```nix
{ config, pkgs, ... }:

{
  systemd.services.my-app = {
    description = "My Application Server";
    wantedBy = [ "multi-user.target" ];
    after = [ "network.target" "postgresql.service" ];
    wants = [ "postgresql.service" ];

    environment = {
      NODE_ENV = "production";
      PORT = "3000";
    };

    serviceConfig = {
      Type = "simple";
      User = "myapp";
      Group = "myapp";
      WorkingDirectory = "/var/lib/myapp";
      ExecStart = "${pkgs.nodejs}/bin/node /var/lib/myapp/server.js";

      Restart = "always";
      RestartSec = 10;

      # Logging
      StandardOutput = "journal";
      StandardError = "journal";
      SyslogIdentifier = "my-app";
    };
  };

  # Create the user
  users.users.myapp = {
    isSystemUser = true;
    group = "myapp";
    home = "/var/lib/myapp";
    createHome = true;
  };

  users.groups.myapp = {};
}
```

## Service with Secrets

Using secrets from agenix or sops-nix:

```nix
{ config, pkgs, ... }:

{
  # Declare the secret
  age.secrets.app-api-key = {
    file = ./secrets/app-api-key.age;
    owner = "myapp";
    mode = "0400";
  };

  systemd.services.my-app = {
    description = "My Application";
    wantedBy = [ "multi-user.target" ];
    after = [ "network.target" ];

    # Load secrets from environment file
    serviceConfig = {
      Type = "simple";
      User = "myapp";
      EnvironmentFile = config.age.secrets.app-api-key.path;
      ExecStart = "${pkgs.myapp}/bin/myapp";
    };
  };
}
```

## Hardened Service

Security-hardened service configuration:

```nix
{ config, pkgs, ... }:

{
  systemd.services.secure-app = {
    description = "Security-Hardened Application";
    wantedBy = [ "multi-user.target" ];
    after = [ "network.target" ];

    serviceConfig = {
      Type = "simple";
      User = "secureapp";
      Group = "secureapp";

      ExecStart = "${pkgs.myapp}/bin/myapp";

      # Restart policy
      Restart = "on-failure";
      RestartSec = 5;

      # Security hardening
      NoNewPrivileges = true;
      PrivateTmp = true;
      PrivateDevices = true;
      ProtectSystem = "strict";
      ProtectHome = true;
      ProtectKernelTunables = true;
      ProtectKernelModules = true;
      ProtectControlGroups = true;

      # Restrict capabilities
      CapabilityBoundingSet = "";
      AmbientCapabilities = "";

      # Restrict namespaces
      RestrictNamespaces = true;

      # Restrict address families
      RestrictAddressFamilies = [ "AF_INET" "AF_INET6" "AF_UNIX" ];

      # Restrict system calls
      SystemCallFilter = [ "@system-service" "~@privileged" "~@resources" ];
      SystemCallArchitectures = "native";

      # Memory restrictions
      MemoryDenyWriteExecute = true;

      # Lock personality
      LockPersonality = true;

      # Read-only paths
      ReadOnlyPaths = [ "/" ];
      ReadWritePaths = [ "/var/lib/secureapp" ];

      # Inaccessible paths
      InaccessiblePaths = [ "/home" "/root" ];
    };
  };
}
```

## Oneshot Service (Run Once)

For tasks that run once and exit:

```nix
{ config, pkgs, ... }:

{
  systemd.services.database-migration = {
    description = "Run Database Migrations";
    wantedBy = [ "multi-user.target" ];
    after = [ "postgresql.service" ];
    requires = [ "postgresql.service" ];

    # Only run once on boot, not on restart
    unitConfig = {
      ConditionPathExists = "!/var/lib/myapp/.migrations-done";
    };

    serviceConfig = {
      Type = "oneshot";
      User = "myapp";
      WorkingDirectory = "/var/lib/myapp";
      ExecStart = "${pkgs.myapp}/bin/migrate";
      ExecStartPost = "${pkgs.coreutils}/bin/touch /var/lib/myapp/.migrations-done";
      RemainAfterExit = true;
    };
  };
}
```

## Timer Service (Scheduled Tasks)

Run services on a schedule:

```nix
{ config, pkgs, ... }:

{
  # The service to run
  systemd.services.backup = {
    description = "Backup Service";

    serviceConfig = {
      Type = "oneshot";
      User = "root";
      ExecStart = "${pkgs.writeShellScript "backup" ''
        ${pkgs.restic}/bin/restic backup /var/lib/important-data
      ''}";
    };

    # Don't start on boot, only via timer
    # wantedBy is not set
  };

  # The timer
  systemd.timers.backup = {
    description = "Daily Backup Timer";
    wantedBy = [ "timers.target" ];

    timerConfig = {
      # Run daily at 3 AM
      OnCalendar = "*-*-* 03:00:00";

      # Run if missed (e.g., system was off)
      Persistent = true;

      # Random delay up to 30 minutes
      RandomizedDelaySec = "30min";
    };
  };
}
```

## Socket-Activated Service

Start service on-demand when connection arrives:

```nix
{ config, pkgs, ... }:

{
  systemd.sockets.my-api = {
    description = "My API Socket";
    wantedBy = [ "sockets.target" ];

    socketConfig = {
      ListenStream = "0.0.0.0:8080";
      Accept = false;
    };
  };

  systemd.services.my-api = {
    description = "My API Service";
    requires = [ "my-api.socket" ];
    after = [ "my-api.socket" ];

    serviceConfig = {
      Type = "simple";
      User = "myapi";
      ExecStart = "${pkgs.myapi}/bin/myapi";

      # Receive socket from systemd
      StandardInput = "socket";
    };
  };
}
```

## Service with Health Check

```nix
{ config, pkgs, ... }:

{
  systemd.services.my-app = {
    description = "My Application with Health Check";
    wantedBy = [ "multi-user.target" ];
    after = [ "network.target" ];

    serviceConfig = {
      Type = "notify";  # Service notifies systemd when ready
      User = "myapp";
      ExecStart = "${pkgs.myapp}/bin/myapp";

      # Health check
      ExecStartPost = "${pkgs.writeShellScript "health-check" ''
        for i in $(seq 1 30); do
          if ${pkgs.curl}/bin/curl -sf http://localhost:3000/health; then
            exit 0
          fi
          sleep 1
        done
        exit 1
      ''}";

      # Watchdog
      WatchdogSec = 30;

      Restart = "on-failure";
      RestartSec = 10;
    };
  };
}
```

## Resource Limits

```nix
{ config, pkgs, ... }:

{
  systemd.services.resource-limited = {
    description = "Resource-Limited Service";
    wantedBy = [ "multi-user.target" ];

    serviceConfig = {
      Type = "simple";
      ExecStart = "${pkgs.myapp}/bin/myapp";

      # Memory limits
      MemoryMax = "512M";
      MemoryHigh = "400M";

      # CPU limits
      CPUQuota = "50%";
      CPUWeight = 100;

      # IO limits
      IOWeight = 100;

      # Process limits
      TasksMax = 50;

      # File descriptor limits
      LimitNOFILE = 65536;
    };
  };
}
```

## Managing Services

```bash
# Start/stop/restart
sudo systemctl start my-app
sudo systemctl stop my-app
sudo systemctl restart my-app

# Enable/disable on boot
sudo systemctl enable my-app
sudo systemctl disable my-app

# Check status
sudo systemctl status my-app

# View logs
sudo journalctl -u my-app -f
sudo journalctl -u my-app --since "1 hour ago"

# List timers
sudo systemctl list-timers

# Reload systemd (after NixOS rebuild)
sudo systemctl daemon-reload
```
