use clap::{Parser, Subcommand};

mod docs;
mod hyprland;
mod nix;
mod nixos;
mod paths;
mod vault;
mod vaultwarden;
mod wireguard;

#[derive(Parser)]
#[command(name = "syntek-infra-tool")]
#[command(author = "Syntek Developers")]
#[command(version = "0.1.0")]
#[command(about = "CLI tool for NixOS and Wireguard environment detection")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Hyprland wayland compositor helpers
    Hyprland {
        #[command(subcommand)]
        action: HyprlandCommands,
    },
    /// Nix package manager detection and helpers
    Nix {
        #[command(subcommand)]
        action: NixCommands,
    },
    /// NixOS system information and management
    Nixos {
        #[command(subcommand)]
        action: NixosCommands,
    },
    /// Hashicorp Vault integration
    Vault {
        #[command(subcommand)]
        action: VaultCommands,
    },
    /// Vaultwarden integration
    Vaultwarden {
        #[command(subcommand)]
        action: VaultwardenCommands,
    },
    /// Wireguard helpers
    Wireguard {
        #[command(subcommand)]
        action: WireguardCommands,
    },
    /// Project documentation helpers (.claude/ files)
    Docs {
        #[command(subcommand)]
        action: DocsCommands,
    },
    /// Resolve plugin directory paths (device-independent)
    Paths {
        #[command(subcommand)]
        action: PathsCommands,
    },
}

#[derive(Subcommand)]
enum HyprlandCommands {
    /// Detect Hyprland installation and configuration
    Detect,
    /// Get current Hyprland status (monitors, workspaces, etc.)
    Status,
    /// Validate a Hyprland configuration file
    Validate {
        /// Path to hyprland.conf
        #[arg(long, default_value = "~/.config/hypr/hyprland.conf")]
        path: String,
    },
    /// Reload Hyprland configuration
    Reload,
}

#[derive(Subcommand)]
enum NixCommands {
    /// Detect Nix installation, version, and flakes support
    Detect,
    /// List configured Nix channels
    Channels,
    /// Validate a Nix flake
    FlakeCheck {
        /// Path to flake directory
        #[arg(long, default_value = ".")]
        path: String,
    },
}

#[derive(Subcommand)]
enum NixosCommands {
    /// Get current NixOS generation and system status
    Status,
    /// List NixOS generations for rollback
    Generations,
    /// Validate a NixOS configuration
    Validate {
        /// Path to configuration directory
        #[arg(long, default_value = ".")]
        path: String,
    },
}

#[derive(Subcommand)]
enum VaultCommands {
    /// Check Vault connectivity and authentication status
    Status,
    /// Read a secret from Vault
    Read {
        /// Secret path
        #[arg(long)]
        path: String,
    },
    /// Write a secret to Vault
    Write {
        /// Secret path
        #[arg(long)]
        path: String,
        /// Secret data as JSON
        #[arg(long)]
        data: String,
    },
}

#[derive(Subcommand)]
enum VaultwardenCommands {
    /// Check Vaultwarden connectivity
    Status,
    /// Sync secrets from Vault to Vaultwarden
    Sync,
}

#[derive(Subcommand)]
enum WireguardCommands {
    /// Generate a Wireguard key pair
    Keygen,
    /// Generate a QR code for mobile configuration
    Qr {
        /// Path to Wireguard configuration file
        #[arg(long)]
        config: String,
    },
}

#[derive(Subcommand)]
enum DocsCommands {
    /// List project documentation files in .claude/
    List,
    /// Show the content of a named documentation file
    Show {
        /// Document name: CODING-PRINCIPLES, TESTING, SECURITY, DEVELOPMENT
        name: String,
    },
}

#[derive(Subcommand)]
enum PathsCommands {
    /// Show all resolved plugin directory paths
    All,
    /// Get a specific plugin directory by name
    Get {
        /// Name: plugin-root, templates, examples, agents, commands
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Hyprland { action } => match action {
            HyprlandCommands::Detect => hyprland::detect(),
            HyprlandCommands::Status => hyprland::status(),
            HyprlandCommands::Validate { path } => hyprland::validate(&path),
            HyprlandCommands::Reload => hyprland::reload(),
        },
        Commands::Nix { action } => match action {
            NixCommands::Detect => nix::detect(),
            NixCommands::Channels => nix::channels(),
            NixCommands::FlakeCheck { path } => nix::flake_check(&path),
        },
        Commands::Nixos { action } => match action {
            NixosCommands::Status => nixos::status(),
            NixosCommands::Generations => nixos::generations(),
            NixosCommands::Validate { path } => nixos::validate(&path),
        },
        Commands::Vault { action } => match action {
            VaultCommands::Status => vault::status(),
            VaultCommands::Read { path } => vault::read(&path),
            VaultCommands::Write { path, data } => vault::write(&path, &data),
        },
        Commands::Vaultwarden { action } => match action {
            VaultwardenCommands::Status => vaultwarden::status(),
            VaultwardenCommands::Sync => vaultwarden::sync(),
        },
        Commands::Wireguard { action } => match action {
            WireguardCommands::Keygen => wireguard::keygen(),
            WireguardCommands::Qr { config } => wireguard::qr(&config),
        },
        Commands::Docs { action } => match action {
            DocsCommands::List => docs::list(),
            DocsCommands::Show { name } => docs::show(&name),
        },
        Commands::Paths { action } => match action {
            PathsCommands::All => paths::all(),
            PathsCommands::Get { name } => paths::get(&name),
        },
    };

    match result {
        Ok(json) => println!("{}", json),
        Err(e) => {
            eprintln!(
                "{}",
                serde_json::json!({
                    "error": true,
                    "message": e.to_string()
                })
            );
            std::process::exit(1);
        }
    }
}
