use crate::config::JavaVersion;
use anyhow::Result;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

pub struct JavaDetector;

impl JavaDetector {
    /// Scan common installation directories for Java installations
    pub fn scan_system() -> Result<Vec<JavaVersion>> {
        let mut versions = Vec::new();
        let search_paths = Self::get_search_paths();

        println!("Scanning system for Java installations...");

        // First, try to find Java via PATH
        if let Ok(path_versions) = Self::detect_from_path() {
            versions.extend(path_versions);
        }

        // Then scan directories
        for base_path in search_paths {
            if base_path.exists() {
                println!("  Scanning: {}", base_path.display());
                versions.extend(Self::scan_directory(&base_path)?);
            }
        }

        // Deduplicate by path
        versions.sort_by(|a, b| a.path.cmp(&b.path));
        versions.dedup_by(|a, b| a.path == b.path);

        Ok(versions)
    }

    /// Detect Java installations from system PATH
    fn detect_from_path() -> Result<Vec<JavaVersion>> {
        let mut versions = Vec::new();

        if cfg!(windows) {
            // Try 'where java' command
            if let Ok(output) = Command::new("where").arg("java").output() {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    for line in stdout.lines() {
                        let java_path = PathBuf::from(line.trim());
                        if java_path.exists() {
                            // Get the parent directory twice to get JDK root (bin -> jdk)
                            if let Some(parent) = java_path.parent().and_then(|p| p.parent()) {
                                if Self::is_jdk_root(parent) {
                                    if let Ok(version) = Self::detect_version(parent) {
                                        versions.push(version);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Also try 'java -version' directly
            if let Ok(output) = Command::new("java").arg("-version").output() {
                if output.status.success() {
                    // Try to find JAVA_HOME
                    if let Ok(java_home) = std::env::var("JAVA_HOME") {
                        let java_home_path = PathBuf::from(java_home);
                        if Self::is_jdk_root(&java_home_path) {
                            if let Ok(version) = Self::detect_version(&java_home_path) {
                                versions.push(version);
                            }
                        }
                    }
                }
            }
        } else {
            // Unix: use 'which java'
            if let Ok(output) = Command::new("which").arg("java").output() {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let java_path = PathBuf::from(stdout.trim());
                    if let Some(parent) = java_path.parent().and_then(|p| p.parent()) {
                        if Self::is_jdk_root(parent) {
                            if let Ok(version) = Self::detect_version(parent) {
                                versions.push(version);
                            }
                        }
                    }
                }
            }
        }

        Ok(versions)
    }

    /// Get all available drive letters on Windows
    #[cfg(windows)]
    fn get_available_drives() -> Vec<String> {
        let mut drives = Vec::new();
        for letter in 'A'..='Z' {
            let drive = format!("{}:\\", letter);
            let path = PathBuf::from(&drive);
            if path.exists() {
                drives.push(drive);
            }
        }
        drives
    }

    fn get_search_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();

        if cfg!(windows) {
            // Get all available drives
            let drives = Self::get_available_drives();
            
            for drive in drives {
                // Common Java installation paths for each drive
                paths.push(PathBuf::from(&drive).join("Program Files").join("Java"));
                paths.push(PathBuf::from(&drive).join("Program Files").join("Eclipse Adoptium"));
                paths.push(PathBuf::from(&drive).join("Program Files").join("Eclipse Foundation"));
                paths.push(PathBuf::from(&drive).join("Program Files").join("Amazon Corretto"));
                paths.push(PathBuf::from(&drive).join("Program Files").join("Zulu"));
                paths.push(PathBuf::from(&drive).join("Program Files").join("BellSoft"));
                paths.push(PathBuf::from(&drive).join("Program Files").join("Microsoft"));
                paths.push(PathBuf::from(&drive).join("Program Files").join("GraalVM"));
                paths.push(PathBuf::from(&drive).join("Program Files").join("Azul"));
                paths.push(PathBuf::from(&drive).join("Program Files").join("Liberica"));
                
                // Also check Program Files (x86)
                paths.push(PathBuf::from(&drive).join("Program Files (x86)").join("Java"));
                paths.push(PathBuf::from(&drive).join("Program Files (x86)").join("Eclipse Adoptium"));
            }
        } else if cfg!(unix) {
            // Unix/Linux common paths
            paths.push(PathBuf::from("/usr/lib/jvm"));
            paths.push(PathBuf::from("/usr/java"));
            paths.push(PathBuf::from("/opt/java"));
            paths.push(PathBuf::from("/Library/Java/JavaVirtualMachines"));

            // User-installed
            if let Some(home) = dirs::home_dir() {
                paths.push(home.join(".sdkman").join("candidates").join("java"));
                paths.push(home.join(".jenv").join("versions"));
            }
        }

        paths
    }

    fn scan_directory(path: &PathBuf) -> Result<Vec<JavaVersion>> {
        let mut versions = Vec::new();

        for entry in WalkDir::new(path)
            .max_depth(3)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();

            // Check if this looks like a JDK root
            if Self::is_jdk_root(entry_path) {
                if let Ok(version) = Self::detect_version(entry_path) {
                    versions.push(version);
                }
            }
        }

        Ok(versions)
    }

    fn is_jdk_root(path: &std::path::Path) -> bool {
        let bin_dir = path.join("bin");
        if !bin_dir.exists() {
            return false;
        }

        let java_exe = if cfg!(windows) {
            bin_dir.join("java.exe")
        } else {
            bin_dir.join("java")
        };

        java_exe.exists()
    }

    fn detect_version(path: &std::path::Path) -> Result<JavaVersion> {
        let java_exe = if cfg!(windows) {
            path.join("bin").join("java.exe")
        } else {
            path.join("bin").join("java")
        };

        // Execute java -version to get version info
        let output = Command::new(&java_exe).arg("-version").output()?;

        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let version_output = format!("{}{}", stdout, stderr);

        let (version, vendor) = Self::parse_version_output(&version_output)?;
        let is_lts = Self::is_lts_version(&version);
        let architecture = Self::detect_architecture(&version_output);

        Ok(JavaVersion::new(
            version,
            vendor,
            path.to_path_buf(),
            is_lts,
            architecture,
            true, // auto_detected = true
        ))
    }

    fn parse_version_output(output: &str) -> Result<(String, String)> {
        // Match patterns like:
        // java version "1.8.0_292"
        // openjdk version "11.0.12" 2021-07-20
        // java version "17.0.1" 2021-10-19 LTS

        let version_re = Regex::new(r#"(?:java|openjdk) version "([^"]+)""#)?;
        let vendor_re = Regex::new(
            r"(?i)(openjdk|oracle|adoptium|eclipse|corretto|amazon|zulu|azul|graalvm|bellsoft|liberica|microsoft|temurin)",
        )?;

        let version = version_re
            .captures(output)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| anyhow::anyhow!("Could not parse version"))?;

        let vendor = vendor_re
            .captures(output)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        // Normalize old version format (1.8.0_292 -> 8.0.292)
        let normalized_version = if version.starts_with("1.") {
            version.replacen("1.", "", 1)
        } else {
            version
        };

        Ok((normalized_version, vendor))
    }

    fn is_lts_version(version: &str) -> bool {
        // Extract major version number
        let major_version = version
            .split('.')
            .next()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);

        // LTS versions: 8, 11, 17, 21
        matches!(major_version, 8 | 11 | 17 | 21)
    }

    fn detect_architecture(output: &str) -> String {
        if output.contains("64-Bit") || output.contains("x86_64") || output.contains("amd64") {
            "x64".to_string()
        } else if output.contains("aarch64") || output.contains("arm64") {
            "arm64".to_string()
        } else if output.contains("x86") || output.contains("i386") {
            "x86".to_string()
        } else {
            std::env::consts::ARCH.to_string()
        }
    }

    /// Verify a Java installation is valid
    pub fn verify_installation(path: &Path) -> Result<bool> {
        let java_exe = if cfg!(windows) {
            path.join("bin").join("java.exe")
        } else {
            path.join("bin").join("java")
        };

        if !java_exe.exists() {
            return Ok(false);
        }

        // Try to run java -version
        let result = Command::new(&java_exe).arg("-version").output();

        Ok(result.is_ok())
    }
}
