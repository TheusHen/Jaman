mod commands;
mod config;
mod detector;
mod downloader;
mod path_manager;

use anyhow::Result;
use clap::{Parser, Subcommand};
use console::style;

use commands::{
    ActivateCommand, CleanCommand, DoctorCommand, InstallCommand, ListCommand, ScanCommand,
};

#[derive(Parser)]
#[command(name = "jaman")]
#[command(version = "0.1.0")]
#[command(about = "A powerful Java version manager", long_about = None)]
#[command(author = "Jaman Contributors")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List Java versions
    #[command(visible_alias = "ls")]
    List {
        /// Filter versions by number (e.g., "21" shows all 21.x versions)
        filter: Option<String>,

        /// List available versions that can be downloaded
        #[arg(long)]
        available: bool,

        /// List only LTS versions
        #[arg(long)]
        lts: bool,

        /// List only GraalVM versions
        #[arg(long)]
        graalvm: bool,
    },

    /// Install a Java version
    #[command(visible_alias = "i")]
    Install {
        /// Version to install (e.g., "21", "17.0.1", or leave empty for interactive selection)
        version: Option<String>,
    },

    /// Activate a Java version
    #[command(visible_alias = "use")]
    Activate {
        /// Version to activate (or leave empty for interactive selection)
        version: Option<String>,
    },

    /// Scan system for existing Java installations
    Scan,

    /// Run diagnostics on jaman configuration
    Doctor,

    /// Remove unused Java installations
    Clean {
        /// Remove versions not used in the last N days (default: 90)
        #[arg(long, default_value = "90")]
        days: Option<i64>,

        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Show jaman status and information
    Status,

    /// Configure jaman settings
    Config {
        /// Set installation directory
        #[arg(long)]
        set_install_dir: Option<String>,

        /// Set download directory
        #[arg(long)]
        set_download_dir: Option<String>,

        /// Show current configuration
        #[arg(long)]
        show: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Check and add jaman to PATH on first run
    ensure_jaman_in_path()?;

    let cli = Cli::parse();

    // If no command provided, show status
    match cli.command {
        None => show_status().await?,
        Some(Commands::List {
            filter,
            available,
            lts,
            graalvm,
        }) => ListCommand::execute(filter, available, lts, graalvm).await?,
        Some(Commands::Install { version }) => InstallCommand::execute(version).await?,
        Some(Commands::Activate { version }) => ActivateCommand::execute(version).await?,
        Some(Commands::Scan) => ScanCommand::execute().await?,
        Some(Commands::Doctor) => DoctorCommand::execute().await?,
        Some(Commands::Clean { days, force }) => CleanCommand::execute(days, force).await?,
        Some(Commands::Status) => show_status().await?,
        Some(Commands::Config {
            set_install_dir,
            set_download_dir,
            show,
        }) => handle_config(set_install_dir, set_download_dir, show)?,
    }

    Ok(())
}

async fn show_status() -> Result<()> {
    use config::Config;
    use path_manager::PathManager;

    println!(
        "{}",
        style("╔═══════════════════════════════════════════╗").cyan()
    );
    println!(
        "{}",
        style("║          JAMAN - Java Manager           ║")
            .cyan()
            .bold()
    );
    println!(
        "{}",
        style("╚═══════════════════════════════════════════╝").cyan()
    );
    println!();

    let config = Config::load()?;

    // Active version
    if let Some(active) = &config.active_version {
        println!(
            "{} {}",
            style("Active Version:").bold(),
            style(active).green().bold()
        );

        if let Some(version) = config.get_version(active) {
            println!("  Vendor:       {}", style(&version.vendor).cyan());
            println!("  Path:         {}", style(version.path.display()).dim());
            println!("  Architecture: {}", style(&version.architecture).dim());
        }
    } else {
        println!(
            "{} {}",
            style("Active Version:").bold(),
            style("None").yellow()
        );
    }

    println!();

    // Installation info
    println!("{}", style("Installation:").bold());
    println!(
        "  Tracked versions:  {}",
        style(config.installed_versions.len()).cyan()
    );
    println!(
        "  Installation dir:  {}",
        style(config.installation_dir.display()).dim()
    );
    println!(
        "  Download dir:      {}",
        style(config.download_dir.display()).dim()
    );

    // Last scan
    if let Some(last_scan) = config.last_scan {
        println!(
            "  Last scan:         {}",
            style(last_scan.format("%Y-%m-%d %H:%M")).dim()
        );
    }

    println!();

    // Java in PATH
    if let Some(java_home) = PathManager::get_current_java_home() {
        println!(
            "{} {}",
            style("JAVA_HOME:").bold(),
            style(java_home.display()).dim()
        );
    } else {
        println!(
            "{} {}",
            style("JAVA_HOME:").bold(),
            style("Not set").yellow()
        );
    }

    println!();
    println!("{}", style("─".repeat(60)).dim());
    println!("{}", style("Quick commands:").bold());
    println!("  {} - List installed versions", style("jaman list").cyan());
    println!(
        "  {} - List available downloads",
        style("jaman list --available").cyan()
    );
    println!(
        "  {} - Install a version",
        style("jaman install <version>").cyan()
    );
    println!(
        "  {} - Activate a version",
        style("jaman activate <version>").cyan()
    );
    println!(
        "  {} - Scan for existing installations",
        style("jaman scan").cyan()
    );

    Ok(())
}

fn handle_config(set_install_dir: Option<String>, set_download_dir: Option<String>, show: bool) -> Result<()> {
    use config::Config;
    use std::path::PathBuf;

    if show {
        let config = Config::load()?;
        let config_file = Config::config_file()?;

        println!("{}", style("Jaman Configuration:").bold().green());
        println!();
        println!(
            "  Config file:       {}",
            style(config_file.display()).cyan()
        );
        println!(
            "  Installation dir:  {}",
            style(config.installation_dir.display()).cyan()
        );
        println!(
            "  Download dir:      {}",
            style(config.download_dir.display()).cyan()
        );
        println!(
            "  Tracked versions:  {}",
            style(config.installed_versions.len()).cyan()
        );

        if let Some(active) = config.active_version {
            println!("  Active version:    {}", style(active).green());
        }

        return Ok(());
    }

    let mut config = Config::load()?;
    let mut updated = false;

    if let Some(dir_path) = set_install_dir {
        let new_path = PathBuf::from(dir_path);

        if !new_path.exists() {
            std::fs::create_dir_all(&new_path)?;
        }

        config.installation_dir = new_path.clone();
        updated = true;

        println!(
            "{} Installation directory set to: {}",
            style("✓").green().bold(),
            style(new_path.display()).cyan()
        );
    }

    if let Some(dir_path) = set_download_dir {
        let new_path = PathBuf::from(dir_path);

        if !new_path.exists() {
            std::fs::create_dir_all(&new_path)?;
        }

        config.download_dir = new_path.clone();
        updated = true;

        println!(
            "{} Download directory set to: {}",
            style("✓").green().bold(),
            style(new_path.display()).cyan()
        );
    }

    if updated {
        config.save()?;
    }

    Ok(())
}

/// Ensure jaman is added to PATH on first run
fn ensure_jaman_in_path() -> Result<()> {
    use path_manager::PathManager;
    use config::Config;

    // Check if this is the first run by checking if config exists
    let config_file = Config::config_file()?;
    let is_first_run = !config_file.exists();

    // Always check if jaman is in PATH and add if not
    if !PathManager::is_jaman_in_path() {
        println!("{}", style("Setting up jaman...").cyan().bold());
        println!("{}", style("Adding jaman to system PATH...").dim());
        
        if let Err(e) = PathManager::add_jaman_to_path() {
            eprintln!(
                "{} Failed to add jaman to PATH: {}",
                style("⚠").yellow(),
                e
            );
            eprintln!(
                "{}",
                style("You may need to add jaman to PATH manually.").yellow()
            );
        } else {
            println!(
                "{} {}",
                style("✓").green().bold(),
                style("jaman added to PATH successfully!").green()
            );
            
            if cfg!(windows) {
                println!(
                    "{}",
                    style("You can now use 'jaman' command in new terminal windows.").dim()
                );
            } else {
                println!(
                    "{}",
                    style("Please restart your terminal or run: source ~/.bashrc").dim()
                );
            }
            
            if is_first_run {
                println!();
                println!("{}", style("Welcome to jaman! 🎉").green().bold());
                println!("{}", style("Run 'jaman --help' to get started.").dim());
            }
        }
        println!();
    }

    Ok(())
}
