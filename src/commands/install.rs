use anyhow::Result;
use console::style;
use dialoguer::Select;
use crate::config::{Config, JavaVersion};
use crate::downloader::Downloader;

pub struct InstallCommand;

impl InstallCommand {
    pub async fn execute(version_query: Option<String>) -> Result<()> {
        let mut config = Config::load()?;
        let downloader = Downloader::new();

        println!("{}", style("Fetching available versions...").dim());
        let available_versions = downloader.fetch_available_versions().await?;

        if available_versions.is_empty() {
            anyhow::bail!("No versions available for download");
        }

        // Select version
        let selected_version = if let Some(query) = version_query {
            // Find matching version
            available_versions
                .iter()
                .find(|v| v.version.contains(&query))
                .ok_or_else(|| anyhow::anyhow!("Version {} not found", query))?
                .clone()
        } else {
            // Interactive selection
            let version_names: Vec<String> = available_versions
                .iter()
                .map(|v| {
                    let lts = if v.is_lts { " [LTS]" } else { "" };
                    format!("{} - {}{}", v.version, v.vendor, lts)
                })
                .collect();

            let selection = Select::new()
                .with_prompt("Select a Java version to install")
                .items(&version_names)
                .default(0)
                .interact()?;

            available_versions[selection].clone()
        };

        // Check if already installed
        if config.get_version(&selected_version.version).is_some() {
            println!(
                "{}",
                style(format!("Version {} is already installed", selected_version.version))
                    .yellow()
            );
            return Ok(());
        }

        println!(
            "\n{} {} from {}...\n",
            style("Installing").green().bold(),
            style(&selected_version.version).cyan(),
            style(&selected_version.vendor).dim()
        );

        // Download and install
        let jdk_path = downloader
            .download_and_install(&selected_version, &config.installation_dir)
            .await?;

        // Add to config
        let java_version = JavaVersion::new(
            selected_version.version.clone(),
            selected_version.vendor.clone(),
            jdk_path,
            selected_version.is_lts,
            selected_version.architecture.clone(),
            false,
        );

        config.add_version(java_version);
        config.save()?;

        println!(
            "\n{} Successfully installed {} ✓",
            style("✓").green().bold(),
            style(&selected_version.version).cyan().bold()
        );

        println!(
            "\n{} {}",
            style("Use").dim(),
            style(format!("jaman activate {}", selected_version.version)).cyan()
        );

        Ok(())
    }
}
