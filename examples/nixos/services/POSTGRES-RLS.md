# PostgreSQL with Row-Level Security

**Last Updated:** 05/04/2026
**Version:** 1.0.0
**Maintained By:** Infrastructure Team
**Language:** British English (en_GB)
**Timezone:** Europe/London
**Plugin Scope:** syntek-infra (NixOS, Vault, PostgreSQL)

---

Complete reference for deploying PostgreSQL on NixOS with Row-Level Security
(RLS) enforced, credentials managed by the Vault database secrets engine, and
migrations applied via a versioned migration tool.

---

## Overview

This pattern covers:

1. **NixOS module** — hardened `services.postgresql` configuration
2. **Vault database secrets engine** — dynamic, short-lived credentials per role
3. **PostgreSQL roles** — one role per access pattern, referenced in RLS policies
4. **RLS policies** — default-deny, role-scoped, applied via migrations
5. **Migration service** — versioned SQL migrations as a NixOS systemd oneshot

---

## NixOS PostgreSQL Module

```nix
# modules/database/postgres.nix
{ config, lib, pkgs, ... }:

let
  cfg = config.custom.database.postgres;
in
{
  options.custom.database.postgres = {
    enable = lib.mkEnableOption "PostgreSQL with Row-Level Security";

    database = lib.mkOption {
      type = lib.types.nonEmptyStr;
      description = lib.mdDoc "Name of the application database.";
      example = "myapp";
    };

    vaultConfigName = lib.mkOption {
      type = lib.types.nonEmptyStr;
      default = "postgres-main";
      description = lib.mdDoc "Name of the Vault database secrets engine configuration.";
    };

    migrationPackage = lib.mkOption {
      type = lib.types.package;
      default = pkgs.dbmate;
      description = lib.mdDoc "Migration tool package. Must support SQL migration files.";
    };

    migrationsDir = lib.mkOption {
      type = lib.types.path;
      description = lib.mdDoc "Path to versioned SQL migration files.";
      example = "/etc/myapp/migrations";
    };
  };

  config = lib.mkIf cfg.enable {
    assertions = [
      {
        assertion = config.services.vault-agent.enable;
        message = "custom.database.postgres requires vault-agent for credential injection.";
      }
    ];

    services.postgresql = {
      enable = true;
      package = pkgs.postgresql_16;

      # scram-sha-256 authentication only — trust and md5 are not permitted
      authentication = lib.mkForce ''
        local all postgres peer
        local all all   scram-sha-256
        host  all all   127.0.0.1/32 scram-sha-256
        host  all all   ::1/128      scram-sha-256
      '';

      settings = {
        # Logging — capture connections and DDL for audit
        log_connections    = true;
        log_disconnections = true;
        log_statement      = "ddl";
        log_line_prefix    = "%t [%p]: user=%u,db=%d,app=%a,client=%h ";

        # TLS — enforce for all network connections
        ssl      = true;
        ssl_cert_file = "/run/secrets/postgres/server.crt";
        ssl_key_file  = "/run/secrets/postgres/server.key";

        # Prevent privilege escalation
        superuser_reserved_connections = 3;
      };

      ensureDatabases = [ cfg.database ];
    };

    # Vault agent injects short-lived credentials into /run/secrets/db-creds
    # The migration service reads those credentials at startup
    systemd.services."db-migrate-${cfg.database}" = {
      description = "Versioned database migrations for ${cfg.database}";
      after    = [ "postgresql.service" "vault-agent.service" ];
      requires = [ "postgresql.service" "vault-agent.service" ];
      wantedBy = [ "multi-user.target" ];

      environment = {
        DATABASE_URL = "postgres://\${DB_USERNAME}:\${DB_PASSWORD}@127.0.0.1:5432/${cfg.database}?sslmode=require";
      };

      serviceConfig = {
        Type            = "oneshot";
        RemainAfterExit = true;
        EnvironmentFile = "/run/secrets/db-creds";   # Written by Vault agent
        ExecStart       = "${cfg.migrationPackage}/bin/dbmate --migrations-dir ${cfg.migrationsDir} up";
        User            = "postgres";

        # Hardening
        ProtectSystem    = "strict";
        PrivateTmp       = true;
        NoNewPrivileges  = true;
        CapabilityBoundingSet = "";
        SystemCallFilter = [ "@system-service" ];
      };
    };
  };
}
```

---

## Vault Database Secrets Engine

Run these commands once during initial Vault setup. Store the superuser
password in Vault before running:

```bash
# Enable the database secrets engine
vault secrets enable database

# Configure the PostgreSQL connection (superuser for credential management only)
vault write database/config/postgres-main \
  plugin_name=postgresql-database-plugin \
  allowed_roles="app-readonly,app-readwrite,app-admin" \
  connection_url="postgresql://{{username}}:{{password}}@127.0.0.1:5432/myapp?sslmode=require" \
  username="vault-superuser" \
  password="$(vault kv get -field=value secret/postgres/superuser-password)"

# Read-only role — SELECT only, TTL 1 hour
vault write database/roles/app-readonly \
  db_name=postgres-main \
  creation_statements="
    CREATE ROLE \"{{name}}\" WITH LOGIN PASSWORD '{{password}}' VALID UNTIL '{{expiration}}';
    GRANT CONNECT ON DATABASE myapp TO \"{{name}}\";
    GRANT SELECT ON ALL TABLES IN SCHEMA public TO \"{{name}}\";
    GRANT app_readonly TO \"{{name}}\";
  " \
  revocation_statements="DROP ROLE IF EXISTS \"{{name}}\";" \
  default_ttl="1h" \
  max_ttl="24h"

# Read-write role — SELECT, INSERT, UPDATE, DELETE; TTL 1 hour
vault write database/roles/app-readwrite \
  db_name=postgres-main \
  creation_statements="
    CREATE ROLE \"{{name}}\" WITH LOGIN PASSWORD '{{password}}' VALID UNTIL '{{expiration}}';
    GRANT CONNECT ON DATABASE myapp TO \"{{name}}\";
    GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO \"{{name}}\";
    GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO \"{{name}}\";
    GRANT app_readwrite TO \"{{name}}\";
  " \
  revocation_statements="DROP ROLE IF EXISTS \"{{name}}\";" \
  default_ttl="1h" \
  max_ttl="24h"

# Admin role — schema migrations only (no direct row access)
vault write database/roles/app-admin \
  db_name=postgres-main \
  creation_statements="
    CREATE ROLE \"{{name}}\" WITH LOGIN PASSWORD '{{password}}' VALID UNTIL '{{expiration}}';
    GRANT CONNECT ON DATABASE myapp TO \"{{name}}\";
    GRANT CREATE ON SCHEMA public TO \"{{name}}\";
    GRANT app_admin TO \"{{name}}\";
  " \
  revocation_statements="DROP ROLE IF EXISTS \"{{name}}\";" \
  default_ttl="30m" \
  max_ttl="1h"
```

**Vault policies** — each service gets only the credentials role it needs:

```hcl
# vault-policies/app-readonly.hcl
path "database/creds/app-readonly" {
  capabilities = ["read"]
}
path "*" {
  capabilities = ["deny"]
}
```

```hcl
# vault-policies/app-readwrite.hcl
path "database/creds/app-readwrite" {
  capabilities = ["read"]
}
path "*" {
  capabilities = ["deny"]
}
```

---

## Vault Agent Template — Credential Injection

The Vault agent writes fresh credentials to `/run/secrets/db-creds` before the
service starts and renews them before they expire:

```nix
# In your NixOS vault-agent configuration
services.vault-agent.settings.template = [
  {
    # Read-only credentials for the reporting service
    source = pkgs.writeText "db-readonly-creds.tpl" ''
      {{ with secret "database/creds/app-readonly" }}
      DB_USERNAME="{{ .Data.username }}"
      DB_PASSWORD="{{ .Data.password }}"
      {{ end }}
    '';
    destination = "/run/secrets/db-creds";
    perms       = "0400";
    # Restart the service when credentials rotate
    command     = "systemctl restart myapp.service";
  }
];
```

---

## PostgreSQL Role Setup

Run this once as the `postgres` superuser during initial database setup. These
are the named PostgreSQL roles that RLS policies reference — separate from the
ephemeral Vault-generated usernames:

```sql
-- Named roles — these never log in directly; Vault-generated users inherit them
CREATE ROLE app_readonly  NOLOGIN NOINHERIT;
CREATE ROLE app_readwrite NOLOGIN NOINHERIT;
CREATE ROLE app_admin     NOLOGIN NOINHERIT;

-- Grant privileges to named roles
GRANT SELECT                           ON ALL TABLES IN SCHEMA public TO app_readonly;
GRANT SELECT, INSERT, UPDATE, DELETE   ON ALL TABLES IN SCHEMA public TO app_readwrite;
GRANT USAGE, SELECT                    ON ALL SEQUENCES IN SCHEMA public TO app_readwrite;
GRANT CREATE                           ON SCHEMA public TO app_admin;

-- Ensure future tables are covered
ALTER DEFAULT PRIVILEGES IN SCHEMA public
  GRANT SELECT ON TABLES TO app_readonly;
ALTER DEFAULT PRIVILEGES IN SCHEMA public
  GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO app_readwrite;
```

---

## RLS Migration Files

RLS policies belong in versioned migration files so they are applied
idempotently and tracked in the migration history.

### Migration: Enable RLS

```sql
-- migrations/20260101000001_enable_rls.sql

-- user_data: personal data — read/write scoped to role membership
ALTER TABLE user_data ENABLE ROW LEVEL SECURITY;
ALTER TABLE user_data FORCE ROW LEVEL SECURITY;

-- audit_log: append-only audit trail — read access limited to admin role
ALTER TABLE audit_log ENABLE ROW LEVEL SECURITY;
ALTER TABLE audit_log FORCE ROW LEVEL SECURITY;

-- tenant_records: multi-tenant data — session variable scoping
ALTER TABLE tenant_records ENABLE ROW LEVEL SECURITY;
ALTER TABLE tenant_records FORCE ROW LEVEL SECURITY;
```

### Migration: RLS Policies

```sql
-- migrations/20260101000002_rls_policies.sql

-- user_data: read policy
-- Any Vault-generated user granted app_readonly can read all rows.
-- Narrow this further if per-user row ownership is required.
CREATE POLICY user_data_read ON user_data
  FOR SELECT
  USING (pg_has_role(current_user, 'app_readonly', 'USAGE'));

-- user_data: write policy
CREATE POLICY user_data_write ON user_data
  FOR INSERT
  WITH CHECK (pg_has_role(current_user, 'app_readwrite', 'USAGE'));

CREATE POLICY user_data_update ON user_data
  FOR UPDATE
  USING     (pg_has_role(current_user, 'app_readwrite', 'USAGE'))
  WITH CHECK (pg_has_role(current_user, 'app_readwrite', 'USAGE'));

CREATE POLICY user_data_delete ON user_data
  FOR DELETE
  USING (pg_has_role(current_user, 'app_readwrite', 'USAGE'));

-- audit_log: append-only (no UPDATE or DELETE policies)
CREATE POLICY audit_log_insert ON audit_log
  FOR INSERT
  WITH CHECK (pg_has_role(current_user, 'app_readwrite', 'USAGE'));

CREATE POLICY audit_log_read ON audit_log
  FOR SELECT
  USING (pg_has_role(current_user, 'app_admin', 'USAGE'));

-- tenant_records: multi-tenant isolation via session variable
-- Application must call:  SET app.tenant_id = '<uuid>';  on every connection
CREATE POLICY tenant_isolation ON tenant_records
  USING (tenant_id = current_setting('app.tenant_id', true)::uuid);
```

---

## Application Connection Pattern

Every application connection must:

1. Obtain fresh credentials from `/run/secrets/db-creds` (written by Vault agent)
2. Set session variables for multi-tenant isolation before executing queries
3. Handle credential expiry and reconnect when Vault agent rotates them

```rust
// Rust example — set tenant session variable on every new connection
use sqlx::postgres::PgPoolOptions;

async fn create_pool(db_url: &str, tenant_id: &str) -> sqlx::PgPool {
    let pool = PgPoolOptions::new()
        .after_connect(move |conn, _meta| {
            let tenant = tenant_id.to_owned();
            Box::pin(async move {
                sqlx::query("SET app.tenant_id = $1")
                    .bind(&tenant)
                    .execute(conn)
                    .await?;
                Ok(())
            })
        })
        .connect(db_url)
        .await
        .expect("Failed to connect to database");
    pool
}
```

---

## Security Checklist

Before deploying a PostgreSQL service with RLS:

- [ ] `ENABLE ROW LEVEL SECURITY` applied to all sensitive tables
- [ ] `FORCE ROW LEVEL SECURITY` applied — table owner is also subject to policies
- [ ] Default-deny verified: a table with RLS and no policy returns zero rows
- [ ] Named PostgreSQL roles created (`app_readonly`, `app_readwrite`, `app_admin`)
- [ ] Vault database secrets engine configured with correct `creation_statements`
- [ ] Vault-generated users granted named PostgreSQL roles in `creation_statements`
- [ ] RLS policies reference role membership (`pg_has_role`), not ephemeral usernames
- [ ] Multi-tenant policies set session variables — application sets them on connect
- [ ] RLS policies applied via versioned migrations, not `initialScript`
- [ ] `scram-sha-256` authentication enforced — no `trust` or `md5`
- [ ] PostgreSQL SSL enabled and enforced for network connections
- [ ] Vault policy grants only `database/creds/<role>` — all other paths denied
- [ ] `nix flake check` passes
- [ ] Configuration validated in a VM before deploying to hardware
