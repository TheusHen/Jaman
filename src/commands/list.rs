use anyhow::Result;
use console::style;
use crate::config::Config;
use crate::downloader::Downloader;

pub struct ListCommand;

impl ListCommand {
    pub async fn execute(filter: Option<String>, available: bool, lts: bool, graalvm: bool) -> Result<()> {
        if available {
            Self::list_available(filter, lts, graalvm).await?;
        } else {
            Self::list_installed(filter, lts)?;
        }
        Ok(())
    }

    fn list_installed(filter: Option<String>, lts_only: bool) -> Result<()> {
        let config = Config::load()?;

        if config.installed_versions.is_empty() {
            println!("{}", style("No Java versions installed yet.").yellow());
            println!("\nUse {} to install a version.", style("jaman install <version>").cyan());
            return Ok(());
        }

        println!("{}\n", style("Installed Java Versions:").bold().green());

        let mut versions = config.installed_versions.clone();
        
        // Apply filters
        if let Some(ref filter_str) = filter {
            versions.retain(|v| v.version.contains(filter_str));
        }

        if lts_only {
            versions.retain(|v| v.is_lts);
        }

        // Sort by version
        versions.sort_by(|a, b| b.version.cmp(&a.version));

        for version in versions {
            let is_active = config.active_version.as_ref().map_or(false, |v| v == &version.version);
            
            let status_icon = if is_active {
                style("●").green().bold()
            } else {
                style("○").dim()
            };

            let version_str = if is_active {
                style(&version.version).green().bold()
            } else {
                style(&version.version).white()
            };

            let lts_badge = if version.is_lts {
                style(" [LTS]").cyan()
            } else {
                style("")
            };

            let auto_detected = if version.auto_detected {
                style(" (auto-detected)").dim()
            } else {
                style("")
            };

            println!(
                "  {} {} - {}{}{} - {}",
                status_icon,
                version_str,
                style(&version.vendor).dim(),
                lts_badge,
                auto_detected,
                style(version.path.display()).dim()
            );
        }

        if let Some(active) = config.active_version {
            println!("\n{} {}", style("Active version:").bold(), style(active).green());
        } else {
            println!("\n{}", style("No active version set.").yellow());
        }

        Ok(())
    }

    async fn list_available(filter: Option<String>, lts_only: bool, graalvm_only: bool) -> Result<()> {
        println!("{}", style("Fetching available versions...").dim());

        let downloader = Downloader::new();
        let mut versions = downloader.fetch_available_versions().await?;

        // Apply filters
        if let Some(ref filter_str) = filter {
            versions.retain(|v| v.version.contains(filter_str));
        }

        if lts_only {
            versions.retain(|v| v.is_lts);
        }

        if graalvm_only {
            versions.retain(|v| v.vendor.to_lowercase().contains("graalvm"));
        }

        if versions.is_empty() {
            println!("{}", style("No versions found matching your criteria.").yellow());
            return Ok(());
        }

        println!("\n{}\n", style("Available Java Versions:").bold().green());

        // Group by major version
        versions.sort_by(|a, b| b.version.cmp(&a.version));

        for version in versions {
            let lts_badge = if version.is_lts {
                style(" [LTS]").cyan()
            } else {
                style("")
            };

            println!(
                "  {} - {}{} - {}",
                style(&version.version).white().bold(),
                style(&version.vendor).dim(),
                lts_badge,
                style(&version.architecture).dim()
            );
        }

        println!("\n{}", style("Use 'jaman install <version>' to install a version.").dim());

        Ok(())
    }
}
