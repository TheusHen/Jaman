use anyhow::Result;
use console::style;
use dialoguer::Confirm;
use chrono::{Duration, Utc};
use std::fs;
use crate::config::Config;

pub struct CleanCommand;

impl CleanCommand {
    pub async fn execute(days: Option<i64>, force: bool) -> Result<()> {
        let mut config = Config::load()?;
        let days_threshold = days.unwrap_or(90);

        println!("{}\n", style(format!("Scanning for unused Java installations (not used in {} days)...", days_threshold)).bold());

        let threshold_date = Utc::now() - Duration::days(days_threshold);
        let mut unused_versions = Vec::new();

        for version in &config.installed_versions {
            // Skip auto-detected versions (we don't manage their files)
            if version.auto_detected {
                continue;
            }

            // Skip active version
            if config.active_version.as_ref().map_or(false, |v| v == &version.version) {
                continue;
            }

            // Check last used date
            let is_unused = match version.last_used {
                Some(last_used) => last_used < threshold_date,
                None => true, // Never used
            };

            if is_unused {
                unused_versions.push(version.clone());
            }
        }

        if unused_versions.is_empty() {
            println!("{}", style("No unused Java installations found.").green());
            return Ok(());
        }

        println!("{} {} unused installation(s) found:\n", 
            style("Found").yellow().bold(),
            style(unused_versions.len()).cyan().bold()
        );

        let mut total_size: u64 = 0;

        for version in &unused_versions {
            let last_used_str = match version.last_used {
                Some(date) => format!("last used {}", date.format("%Y-%m-%d")),
                None => "never used".to_string(),
            };

            let size = Self::calculate_dir_size(&version.path).unwrap_or(0);
            total_size += size;

            println!(
                "  {} {} - {} - {} ({})",
                style("●").yellow(),
                style(&version.version).cyan(),
                style(&version.vendor).dim(),
                style(last_used_str).dim(),
                style(Self::format_size(size)).dim()
            );
        }

        println!("\n{} {}", 
            style("Total space:").bold(),
            style(Self::format_size(total_size)).cyan().bold()
        );

        // Confirm deletion
        let should_delete = if force {
            true
        } else {
            println!();
            Confirm::new()
                .with_prompt("Do you want to remove these installations?")
                .default(false)
                .interact()?
        };

        if !should_delete {
            println!("{}", style("Operation cancelled.").yellow());
            return Ok(());
        }

        println!("\n{}", style("Removing installations...").bold());

        let mut removed_count = 0;
        let mut failed_count = 0;

        for version in unused_versions {
            print!("  Removing {}... ", style(&version.version).cyan());

            match fs::remove_dir_all(&version.path) {
                Ok(_) => {
                    println!("{}", style("✓").green());
                    config.remove_version(&version.path);
                    removed_count += 1;
                }
                Err(e) => {
                    println!("{} {}", style("✗").red(), style(format!("({})", e)).dim());
                    failed_count += 1;
                }
            }
        }

        config.save()?;

        println!("\n{}", style("─".repeat(60)).dim());
        println!(
            "{} {} installation(s) removed",
            style("Summary:").bold(),
            style(removed_count).green().bold()
        );

        if failed_count > 0 {
            println!(
                "  {} installation(s) failed to remove",
                style(failed_count).red()
            );
        }

        println!(
            "  {} freed",
            style(Self::format_size(total_size)).cyan().bold()
        );

        Ok(())
    }

    fn calculate_dir_size(path: &std::path::Path) -> Result<u64> {
        let mut total_size = 0u64;

        if path.is_dir() {
            for entry in walkdir::WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    total_size += entry.metadata()?.len();
                }
            }
        }

        Ok(total_size)
    }

    fn format_size(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if bytes >= GB {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.2} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.2} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} bytes", bytes)
        }
    }
}
