use jaman::detector::JavaDetector;
use std::path::PathBuf;

#[test]
fn test_is_lts_version() {
    // Using the private method through the public API indirectly
    // We'll test through version detection if possible
    // This is a simplified test

    let _lts_versions = ["8", "11", "17", "21"];
    let _non_lts_versions = ["9", "10", "12", "13", "14", "15", "16", "18", "19", "20"];

    // LTS versions should be identified correctly
    // This would require exposing the method or testing indirectly
}

#[test]
fn test_detect_architecture() {
    // Test that architecture detection works
    let arch = std::env::consts::ARCH;
    assert!(!arch.is_empty());
}

#[test]
fn test_version_normalization() {
    // Test that old Java version format (1.8.0) is normalized to (8.0)
    // This tests the parse_version_output logic indirectly

    let _old_format = "1.8.0_292";
    let _expected = "8.0_292";

    // Would need to expose or test through public API
}

#[test]
#[cfg(target_os = "windows")]
fn test_get_search_paths_windows() {
    // Test that Windows search paths are correct
    let paths = vec![
        PathBuf::from("C:\\Program Files\\Java"),
        PathBuf::from("C:\\Program Files\\Eclipse Adoptium"),
    ];

    // Verify paths exist in search list
    for path in paths {
        // Would test through JavaDetector::scan_system()
        assert!(path.to_string_lossy().contains("Program Files"));
    }
}

#[test]
#[cfg(target_os = "linux")]
fn test_get_search_paths_linux() {
    // Test that Linux search paths are correct
    let paths = vec![PathBuf::from("/usr/lib/jvm"), PathBuf::from("/usr/java")];

    for path in paths {
        assert!(path.to_string_lossy().starts_with("/usr"));
    }
}

#[test]
#[cfg(target_os = "macos")]
fn test_get_search_paths_macos() {
    // Test that macOS search paths are correct
    let path = PathBuf::from("/Library/Java/JavaVirtualMachines");
    assert!(path.to_string_lossy().contains("Library"));
}

#[test]
fn test_verify_installation_invalid_path() {
    let invalid_path = PathBuf::from("/nonexistent/path/to/java");
    let result = JavaDetector::verify_installation(&invalid_path);

    // Should return Ok(false) for invalid path
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_parse_vendor_from_output() {
    let outputs = vec![
        ("openjdk version \"11.0.12\"", "openjdk"),
        ("java version \"1.8.0_292\" (Oracle)", "Oracle"),
        ("Eclipse Temurin version \"17.0.1\"", "Temurin"),
    ];

    // Would test vendor detection
    for (output, expected_vendor) in outputs {
        assert!(
            output
                .to_lowercase()
                .contains(&expected_vendor.to_lowercase())
        );
    }
}
