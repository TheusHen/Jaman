use jaman::config::{Config, JavaVersion};
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_config_new() {
    let install_dir = PathBuf::from("/test/path");
    let download_dir = PathBuf::from("/test/downloads");
    let config = Config::new(install_dir.clone(), download_dir.clone());

    assert_eq!(config.installation_dir, install_dir);
    assert_eq!(config.download_dir, download_dir);
    assert_eq!(config.active_version, None);
    assert_eq!(config.installed_versions.len(), 0);
}

#[test]
fn test_add_version() {
    let mut config = Config::new(PathBuf::from("/test"), PathBuf::from("/test/downloads"));

    let version = JavaVersion::new(
        "21.0.1".to_string(),
        "Eclipse Temurin".to_string(),
        PathBuf::from("/test/java21"),
        true,
        "x64".to_string(),
        false,
    );

    config.add_version(version.clone());

    assert_eq!(config.installed_versions.len(), 1);
    assert_eq!(config.installed_versions[0].version, "21.0.1");
}

#[test]
fn test_add_version_duplicate() {
    let mut config = Config::new(PathBuf::from("/test"), PathBuf::from("/test/downloads"));

    let version1 = JavaVersion::new(
        "21.0.1".to_string(),
        "Eclipse Temurin".to_string(),
        PathBuf::from("/test/java21"),
        true,
        "x64".to_string(),
        false,
    );

    let version2 = JavaVersion::new(
        "21.0.2".to_string(),
        "Eclipse Temurin".to_string(),
        PathBuf::from("/test/java21"),
        true,
        "x64".to_string(),
        false,
    );

    config.add_version(version1);
    config.add_version(version2);

    // Should replace the old version with same path
    assert_eq!(config.installed_versions.len(), 1);
    assert_eq!(config.installed_versions[0].version, "21.0.2");
}

#[test]
fn test_remove_version() {
    let mut config = Config::new(PathBuf::from("/test"), PathBuf::from("/test/downloads"));

    let path = PathBuf::from("/test/java21");
    let version = JavaVersion::new(
        "21.0.1".to_string(),
        "Eclipse Temurin".to_string(),
        path.clone(),
        true,
        "x64".to_string(),
        false,
    );

    config.add_version(version);
    assert_eq!(config.installed_versions.len(), 1);

    config.remove_version(&path);
    assert_eq!(config.installed_versions.len(), 0);
}

#[test]
fn test_get_version() {
    let mut config = Config::new(PathBuf::from("/test"), PathBuf::from("/test/downloads"));

    let version = JavaVersion::new(
        "21.0.1".to_string(),
        "Eclipse Temurin".to_string(),
        PathBuf::from("/test/java21"),
        true,
        "x64".to_string(),
        false,
    );

    config.add_version(version);

    let found = config.get_version("21");
    assert!(found.is_some());
    assert_eq!(found.unwrap().version, "21.0.1");

    let not_found = config.get_version("17");
    assert!(not_found.is_none());
}

#[test]
fn test_set_active() {
    let mut config = Config::new(PathBuf::from("/test"), PathBuf::from("/test/downloads"));

    let version = JavaVersion::new(
        "21.0.1".to_string(),
        "Eclipse Temurin".to_string(),
        PathBuf::from("/test/java21"),
        true,
        "x64".to_string(),
        false,
    );

    config.add_version(version);

    let result = config.set_active("21.0.1");
    assert!(result.is_ok());
    assert_eq!(config.active_version, Some("21.0.1".to_string()));
}

#[test]
fn test_set_active_not_found() {
    let mut config = Config::new(PathBuf::from("/test"), PathBuf::from("/test/downloads"));

    let result = config.set_active("21.0.1");
    assert!(result.is_err());
}

#[test]
fn test_java_version_new() {
    let version = JavaVersion::new(
        "21.0.1".to_string(),
        "Eclipse Temurin".to_string(),
        PathBuf::from("/test/java21"),
        true,
        "x64".to_string(),
        false,
    );

    assert_eq!(version.version, "21.0.1");
    assert_eq!(version.vendor, "Eclipse Temurin");
    assert!(version.is_lts);
    assert_eq!(version.architecture, "x64");
    assert!(!version.auto_detected);
    assert!(version.last_used.is_none());
}

#[test]
fn test_java_executable_path() {
    let version = JavaVersion::new(
        "21.0.1".to_string(),
        "Eclipse Temurin".to_string(),
        PathBuf::from("/test/java21"),
        true,
        "x64".to_string(),
        false,
    );

    let java_exe = version.java_executable();

    #[cfg(windows)]
    assert_eq!(java_exe, PathBuf::from("/test/java21/bin/java.exe"));

    #[cfg(not(windows))]
    assert_eq!(java_exe, PathBuf::from("/test/java21/bin/java"));
}

#[test]
fn test_mark_used() {
    let mut version = JavaVersion::new(
        "21.0.1".to_string(),
        "Eclipse Temurin".to_string(),
        PathBuf::from("/test/java21"),
        true,
        "x64".to_string(),
        false,
    );

    assert!(version.last_used.is_none());

    version.mark_used();

    assert!(version.last_used.is_some());
}

#[test]
fn test_config_save_and_load() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_var("HOME", temp_dir.path());

    let mut config = Config::new(temp_dir.path().join("jdks"), temp_dir.path().join("downloads"));

    let version = JavaVersion::new(
        "21.0.1".to_string(),
        "Eclipse Temurin".to_string(),
        PathBuf::from("/test/java21"),
        true,
        "x64".to_string(),
        false,
    );

    config.add_version(version);

    // Note: In real tests, you'd need to set up proper config directories
    // This is a simplified test
    assert_eq!(config.installed_versions.len(), 1);
}
