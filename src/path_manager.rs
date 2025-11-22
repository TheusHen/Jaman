use anyhow::Result;
use std::env;
use std::path::{Path, PathBuf};

#[cfg(windows)]
use winreg::RegKey;
#[cfg(windows)]
use winreg::enums::*;

pub struct PathManager;

impl PathManager {
    /// Set the active Java version by modifying system PATH
    #[cfg(windows)]
    pub fn set_active_java(java_home: &Path) -> Result<()> {
        Self::set_active_java_windows(java_home)
    }

    /// Set the active Java version by modifying system PATH
    #[cfg(not(windows))]
    pub fn set_active_java(java_home: &Path) -> Result<()> {
        Self::set_active_java_unix(java_home)
    }

    #[cfg(windows)]
    fn set_active_java_windows(java_home: &Path) -> Result<()> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let env_key = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;

        // Set JAVA_HOME
        let java_home_str = java_home.to_string_lossy().to_string();
        env_key.set_value("JAVA_HOME", &java_home_str)?;

        // Get current PATH
        let current_path: String = env_key.get_value("Path").unwrap_or_default();

        // Remove any existing Java paths
        let cleaned_path = Self::remove_java_paths(&current_path);

        // Add new Java bin to the beginning of PATH
        let java_bin = java_home.join("bin").to_string_lossy().to_string();
        let new_path = format!("{};{}", java_bin, cleaned_path);

        env_key.set_value("Path", &new_path)?;

        // Broadcast environment change
        Self::broadcast_environment_change();

        Ok(())
    }

    #[cfg(not(windows))]
    fn set_active_java_unix(java_home: &Path) -> Result<()> {
        // For Unix systems, we'll create/update shell configuration files
        let home_dir =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;

        let shell_configs = vec![
            home_dir.join(".bashrc"),
            home_dir.join(".bash_profile"),
            home_dir.join(".zshrc"),
            home_dir.join(".profile"),
        ];

        let java_home_str = java_home.to_string_lossy();
        let export_line = format!(
            "\n# Added by jaman\nexport JAVA_HOME=\"{}\"\nexport PATH=\"$JAVA_HOME/bin:$PATH\"\n",
            java_home_str
        );

        for config_file in shell_configs {
            if config_file.exists() {
                let mut content = std::fs::read_to_string(&config_file)?;

                // Remove old jaman entries
                content = Self::remove_jaman_entries(&content);

                // Add new entry
                content.push_str(&export_line);

                std::fs::write(&config_file, content)?;
            }
        }

        println!("⚠️  Please restart your terminal or run: source ~/.bashrc");

        Ok(())
    }

    #[cfg(windows)]
    fn broadcast_environment_change() {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use std::ptr;

        unsafe {
            const HWND_BROADCAST: isize = 0xffff;
            const WM_SETTINGCHANGE: u32 = 0x001A;
            const SMTO_ABORTIFHUNG: u32 = 0x0002;

            let environment: Vec<u16> = OsStr::new("Environment")
                .encode_wide()
                .chain(Some(0))
                .collect();

            #[link(name = "user32")]
            extern "system" {
                fn SendMessageTimeoutW(
                    hWnd: isize,
                    Msg: u32,
                    wParam: usize,
                    lParam: *const u16,
                    fuFlags: u32,
                    uTimeout: u32,
                    lpdwResult: *mut usize,
                ) -> isize;
            }

            SendMessageTimeoutW(
                HWND_BROADCAST,
                WM_SETTINGCHANGE,
                0,
                environment.as_ptr(),
                SMTO_ABORTIFHUNG,
                5000,
                ptr::null_mut(),
            );
        }
    }

    #[cfg(windows)]
    fn remove_java_paths(path: &str) -> String {
        let paths: Vec<&str> = path.split(';').collect();
        let filtered: Vec<&str> = paths
            .into_iter()
            .filter(|p| {
                let p_lower = p.to_lowercase();
                !p_lower.contains("java") || p_lower.contains("javascript")
            })
            .collect();

        filtered.join(";")
    }

    fn remove_jaman_entries(content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let mut result = Vec::new();
        let mut skip_next = false;

        for line in lines {
            if line.contains("# Added by jaman") {
                skip_next = true;
                continue;
            }

            if skip_next {
                if line.starts_with("export JAVA_HOME") || line.starts_with("export PATH") {
                    continue;
                } else {
                    skip_next = false;
                }
            }

            result.push(line);
        }

        result.join("\n")
    }

    /// Get the current JAVA_HOME
    pub fn get_current_java_home() -> Option<PathBuf> {
        env::var("JAVA_HOME").ok().map(PathBuf::from)
    }

    /// Check if jaman has control over Java PATH
    #[allow(dead_code)]
    pub fn is_jaman_active() -> bool {
        if let Some(java_home) = Self::get_current_java_home() {
            let config = crate::config::Config::load();
            if let Ok(config) = config {
                return config
                    .installed_versions
                    .iter()
                    .any(|v| v.path == java_home);
            }
        }
        false
    }

    /// Remove Java from PATH (deactivate)
    #[cfg(windows)]
    #[allow(dead_code)]
    pub fn deactivate_java() -> Result<()> {
        Self::deactivate_java_windows()
    }

    /// Remove Java from PATH (deactivate)
    #[cfg(not(windows))]
    #[allow(dead_code)]
    pub fn deactivate_java() -> Result<()> {
        Self::deactivate_java_unix()
    }

    #[cfg(windows)]
    fn deactivate_java_windows() -> Result<()> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let env_key = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;

        // Remove JAVA_HOME
        let _ = env_key.delete_value("JAVA_HOME");

        // Clean PATH
        let current_path: String = env_key.get_value("Path").unwrap_or_default();
        let cleaned_path = Self::remove_java_paths(&current_path);
        env_key.set_value("Path", &cleaned_path)?;

        Self::broadcast_environment_change();

        Ok(())
    }

    #[cfg(not(windows))]
    fn deactivate_java_unix() -> Result<()> {
        let home_dir =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;

        let shell_configs = vec![
            home_dir.join(".bashrc"),
            home_dir.join(".bash_profile"),
            home_dir.join(".zshrc"),
            home_dir.join(".profile"),
        ];

        for config_file in shell_configs {
            if config_file.exists() {
                let content = std::fs::read_to_string(&config_file)?;
                let cleaned_content = Self::remove_jaman_entries(&content);
                std::fs::write(&config_file, cleaned_content)?;
            }
        }

        println!("⚠️  Please restart your terminal or run: source ~/.bashrc");

        Ok(())
    }
}
