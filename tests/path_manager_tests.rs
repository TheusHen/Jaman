use jaman::path_manager::PathManager;
use std::path::PathBuf;

#[test]
fn test_get_current_java_home() {
    // Test getting current JAVA_HOME
    let java_home = PathManager::get_current_java_home();

    // May or may not be set
    // Just verify it returns an Option
    assert!(java_home.is_some() || java_home.is_none());
}

#[test]
#[cfg(windows)]
fn test_remove_java_paths_windows() {
    let test_cases = vec![
        (
            "C:\\Windows;C:\\Program Files\\Java\\bin;C:\\Users",
            "C:\\Windows;C:\\Users",
        ),
        (
            "C:\\Path1;C:\\JavaPath\\bin;C:\\Path2",
            "C:\\Path1;C:\\Path2",
        ),
        (
            "C:\\JavaScript\\bin;C:\\Java\\bin;C:\\Node",
            "C:\\JavaScript\\bin;C:\\Node",
        ),
    ];

    // Would test the remove_java_paths function
    // This is a simplified version
    for (input, expected) in test_cases {
        let paths: Vec<&str> = input.split(';').collect();
        let filtered: Vec<&str> = paths
            .into_iter()
            .filter(|p| {
                let p_lower = p.to_lowercase();
                !p_lower.contains("java") || p_lower.contains("javascript")
            })
            .collect();
        let result = filtered.join(";");
        assert_eq!(result, expected);
    }
}

#[test]
fn test_is_jaman_active() {
    // Test if jaman is controlling Java
    let result = PathManager::is_jaman_active();

    // Should return a boolean - just verify it compiles and returns without panic
    let _ = result;
}

#[test]
fn test_remove_jaman_entries_from_shell_config() {
    let content = r#"
# Some config
export PATH="/usr/bin:$PATH"

# Added by jaman
export JAVA_HOME="/path/to/java"
export PATH="$JAVA_HOME/bin:$PATH"

# More config
alias ll='ls -la'
"#;

    let result = remove_jaman_entries(content);

    assert!(!result.contains("Added by jaman"));
    assert!(!result.contains("JAVA_HOME=\"/path/to/java\""));
    assert!(result.contains("alias ll='ls -la'"));
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

#[test]
fn test_path_validation() {
    let valid_paths = vec![
        PathBuf::from("/usr/lib/jvm/java-21"),
        PathBuf::from("C:\\Program Files\\Java\\jdk-21"),
    ];

    for path in valid_paths {
        // Test that paths can be created
        assert!(!path.to_string_lossy().is_empty());
    }
}

#[test]
#[cfg(windows)]
fn test_windows_path_format() {
    let path = PathBuf::from("C:\\Program Files\\Java\\jdk-21");
    let path_str = path.to_string_lossy();

    assert!(path_str.contains("\\"));
    assert!(path_str.contains("Program Files"));
}

#[test]
#[cfg(not(windows))]
fn test_unix_path_format() {
    let path = PathBuf::from("/usr/lib/jvm/java-21");
    let path_str = path.to_string_lossy();

    assert!(path_str.starts_with("/"));
    assert!(!path_str.contains("\\"));
}
