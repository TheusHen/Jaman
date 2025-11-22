use crate::config::Config;
use crate::path_manager::PathManager;
use anyhow::Result;
use console::style;
use dialoguer::Select;

pub struct ActivateCommand;

impl ActivateCommand {
    pub async fn execute(version_query: Option<String>) -> Result<()> {
        let mut config = Config::load()?;

        if config.installed_versions.is_empty() {
            println!("{}", style("No Java versions installed.").yellow());
            println!(
                "\nUse {} to install a version.",
                style("jaman install").cyan()
            );
            return Ok(());
        }

        // Select version
        let selected_version = if let Some(query) = version_query {
            // Find matching version
            config
                .installed_versions
                .iter()
                .find(|v| v.version.contains(&query))
                .ok_or_else(|| anyhow::anyhow!("Version {} not found", query))?
                .clone()
        } else {
            // Interactive selection
            let version_names: Vec<String> = config
                .installed_versions
                .iter()
                .map(|v| {
                    let lts = if v.is_lts { " [LTS]" } else { "" };
                    let active = if config.active_version.as_ref() == Some(&v.version) {
                        " (active)"
                    } else {
                        ""
                    };
                    format!("{} - {}{}{}", v.version, v.vendor, lts, active)
                })
                .collect();

            let selection = Select::new()
                .with_prompt("Select a Java version to activate")
                .items(&version_names)
                .default(0)
                .interact()?;

            config.installed_versions[selection].clone()
        };

        println!(
            "\n{} {}...\n",
            style("Activating").green().bold(),
            style(&selected_version.version).cyan()
        );

        // Set PATH
        PathManager::set_active_java(&selected_version.path)?;

        // Update config
        config.set_active(&selected_version.version)?;

        // Mark as used
        if let Some(version) = config
            .installed_versions
            .iter_mut()
            .find(|v| v.version == selected_version.version)
        {
            version.mark_used();
        }

        config.save()?;

        println!(
            "{} Java {} is now active ✓",
            style("✓").green().bold(),
            style(&selected_version.version).cyan().bold()
        );

        // Verify
        println!("\n{}", style("Verification:").dim());
        println!(
            "  JAVA_HOME: {}",
            style(selected_version.path.display()).cyan()
        );
        println!(
            "  Java bin:  {}",
            style(selected_version.java_executable().display()).cyan()
        );

        if cfg!(windows) {
            println!(
                "\n{}",
                style("Environment variables updated. You may need to restart your terminal.")
                    .yellow()
            );
        }

        Ok(())
    }
}
