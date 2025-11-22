use anyhow::Result;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use chrono::Utc;
use crate::config::Config;
use crate::detector::JavaDetector;

pub struct ScanCommand;

impl ScanCommand {
    pub async fn execute() -> Result<()> {
        let mut config = Config::load()?;

        println!("{}\n", style("Scanning system for Java installations...").bold().green());

        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")?
        );
        pb.set_message("Searching common installation directories...");
        pb.enable_steady_tick(std::time::Duration::from_millis(100));

        // Scan system
        let detected_versions = JavaDetector::scan_system()?;

        pb.finish_and_clear();

        if detected_versions.is_empty() {
            println!("{}", style("No Java installations found on the system.").yellow());
            println!("\nUse {} to install a version.", style("jaman install").cyan());
            return Ok(());
        }

        println!("{} {} Java installation(s)\n", 
            style("Found").green().bold(),
            style(detected_versions.len()).cyan().bold()
        );

        let mut added_count = 0;
        let mut skipped_count = 0;

        for version in detected_versions {
            // Check if already in config
            let already_exists = config.installed_versions.iter().any(|v| v.path == version.path);

            if already_exists {
                println!(
                    "  {} {} - {} {}",
                    style("○").dim(),
                    style(&version.version).dim(),
                    style(&version.vendor).dim(),
                    style("(already tracked)").dim()
                );
                skipped_count += 1;
            } else {
                println!(
                    "  {} {} - {} - {}",
                    style("✓").green(),
                    style(&version.version).cyan(),
                    style(&version.vendor).dim(),
                    style(version.path.display()).dim()
                );
                config.add_version(version);
                added_count += 1;
            }
        }

        // Update last scan time
        config.last_scan = Some(Utc::now());
        config.save()?;

        println!("\n{}", style("─".repeat(60)).dim());
        println!(
            "{} {} new version(s) added, {} already tracked",
            style("Summary:").bold(),
            style(added_count).green().bold(),
            style(skipped_count).dim()
        );

        if added_count > 0 {
            println!("\n{}", style("Use 'jaman list' to see all tracked versions.").dim());
        }

        Ok(())
    }
}
