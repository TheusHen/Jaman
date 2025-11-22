use anyhow::Result;
use console::style;
use std::process::Command;
use crate::config::Config;
use crate::path_manager::PathManager;
use crate::detector::JavaDetector;

pub struct DoctorCommand;

impl DoctorCommand {
    pub async fn execute() -> Result<()> {
        println!("{}\n", style("Running jaman diagnostics...").bold().green());

        let mut issues_found = 0;
        let mut checks_passed = 0;

        // Check 1: Configuration file
        println!("{}", style("Checking configuration...").bold());
        match Config::load() {
            Ok(config) => {
                Self::print_success("Configuration file is valid");
                println!("  Installation directory: {}", style(config.installation_dir.display()).cyan());
                println!("  Tracked versions: {}", style(config.installed_versions.len()).cyan());
                checks_passed += 1;
            }
            Err(e) => {
                Self::print_error(&format!("Configuration error: {}", e));
                issues_found += 1;
            }
        }

        println!();

        // Check 2: Active Java version
        println!("{}", style("Checking active Java...").bold());
        if let Some(java_home) = PathManager::get_current_java_home() {
            Self::print_success(&format!("JAVA_HOME is set: {}", java_home.display()));
            
            // Verify it's valid
            if JavaDetector::verify_installation(&java_home)? {
                Self::print_success("Java installation is valid");
                checks_passed += 1;
            } else {
                Self::print_warning("JAVA_HOME points to invalid Java installation");
                issues_found += 1;
            }
        } else {
            Self::print_warning("JAVA_HOME is not set");
            issues_found += 1;
        }

        println!();

        // Check 3: Java executable accessibility
        println!("{}", style("Checking Java executable...").bold());
        match Command::new("java").arg("-version").output() {
            Ok(output) => {
                if output.status.success() {
                    let version_info = String::from_utf8_lossy(&output.stderr);
                    let first_line = version_info.lines().next().unwrap_or("Unknown");
                    Self::print_success(&format!("Java is accessible: {}", first_line));
                    checks_passed += 1;
                } else {
                    Self::print_error("Java command failed to execute");
                    issues_found += 1;
                }
            }
            Err(_) => {
                Self::print_error("Java command not found in PATH");
                issues_found += 1;
            }
        }

        println!();

        // Check 4: Verify all tracked installations
        println!("{}", style("Verifying tracked installations...").bold());
        let config = Config::load()?;
        let mut valid_count = 0;
        let mut invalid_count = 0;

        for version in &config.installed_versions {
            if JavaDetector::verify_installation(&version.path)? {
                valid_count += 1;
            } else {
                Self::print_warning(&format!("Invalid installation: {} at {}", version.version, version.path.display()));
                invalid_count += 1;
            }
        }

        if invalid_count == 0 {
            Self::print_success(&format!("All {} tracked installation(s) are valid", valid_count));
            checks_passed += 1;
        } else {
            Self::print_warning(&format!("{} invalid installation(s) found", invalid_count));
            issues_found += 1;
        }

        println!();

        // Check 5: Installation directory
        println!("{}", style("Checking installation directory...").bold());
        let config = Config::load()?;
        if config.installation_dir.exists() {
            Self::print_success(&format!("Installation directory exists: {}", config.installation_dir.display()));
            checks_passed += 1;
        } else {
            Self::print_error(&format!("Installation directory not found: {}", config.installation_dir.display()));
            issues_found += 1;
        }

        println!();

        // Summary
        println!("{}", style("─".repeat(60)).dim());
        println!("{}", style("Summary:").bold());
        println!("  {} checks passed", style(checks_passed).green().bold());
        
        if issues_found > 0 {
            println!("  {} issue(s) found", style(issues_found).yellow().bold());
            println!("\n{}", style("Run 'jaman scan' to update tracked installations.").dim());
        } else {
            println!("\n{} {}", style("✓").green().bold(), style("All checks passed!").green());
        }

        Ok(())
    }

    fn print_success(msg: &str) {
        println!("  {} {}", style("✓").green(), msg);
    }

    fn print_warning(msg: &str) {
        println!("  {} {}", style("⚠").yellow(), style(msg).yellow());
    }

    fn print_error(msg: &str) {
        println!("  {} {}", style("✗").red(), style(msg).red());
    }
}
