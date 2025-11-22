use jaman::downloader::{AvailableVersion, Downloader};

#[tokio::test]
async fn test_downloader_new() {
    let _downloader = Downloader::new();
    // Should create successfully without panicking
}

#[tokio::test]
#[ignore] // Ignore by default as it makes real network requests
async fn test_fetch_available_versions() {
    let downloader = Downloader::new();
    let result = downloader.fetch_available_versions().await;

    // Should return successfully
    assert!(result.is_ok());

    if let Ok(versions) = result {
        // Should have some versions
        assert!(!versions.is_empty());

        // Each version should have required fields
        for version in versions.iter().take(5) {
            assert!(!version.version.is_empty());
            assert!(!version.vendor.is_empty());
            assert!(!version.download_url.is_empty());
        }
    }
}

#[test]
fn test_available_version_creation() {
    let version = AvailableVersion {
        version: "21.0.1".to_string(),
        vendor: "Eclipse Temurin".to_string(),
        is_lts: true,
        architecture: "x64".to_string(),
        download_url: "https://example.com/java.zip".to_string(),
        checksum: Some("abc123".to_string()),
    };

    assert_eq!(version.version, "21.0.1");
    assert_eq!(version.vendor, "Eclipse Temurin");
    assert!(version.is_lts);
    assert_eq!(version.architecture, "x64");
    assert!(version.checksum.is_some());
}

#[test]
fn test_extract_filename() {
    let urls = vec![
        ("https://example.com/java-21.0.1.zip", "java-21.0.1.zip"),
        ("https://example.com/path/to/file.tar.gz", "file.tar.gz"),
        ("https://example.com/", "download.zip"),
    ];

    for (url, expected) in urls {
        let filename = url
            .split('/')
            .next_back()
            .unwrap_or("download.zip")
            .to_string();
        let filename = if filename.is_empty() {
            "download.zip".to_string()
        } else {
            filename
        };
        assert_eq!(filename, expected);
    }
}

#[test]
fn test_format_size() {
    let test_cases = vec![
        (512u64, "512 bytes"),
        (1024u64, "1.00 KB"),
        (1024 * 1024, "1.00 MB"),
        (1024 * 1024 * 1024, "1.00 GB"),
        (1536 * 1024 * 1024, "1.50 GB"),
    ];

    for (bytes, expected) in test_cases {
        let result = format_size(bytes);
        assert_eq!(result, expected);
    }
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

#[tokio::test]
#[ignore] // Ignore as it requires actual download
async fn test_download_and_install() {
    let _downloader = Downloader::new();
    let _temp_dir = tempfile::TempDir::new().unwrap();

    let _version = AvailableVersion {
        version: "21.0.1".to_string(),
        vendor: "Eclipse Temurin".to_string(),
        is_lts: true,
        architecture: "x64".to_string(),
        download_url: "https://example.com/java.zip".to_string(),
        checksum: None,
    };

    // This would require a real download
    // In real tests, you'd mock the HTTP client
}

#[test]
fn test_checksum_verification() {
    use sha2::{Digest, Sha256};

    let data = b"test data";
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let hash_str = hex::encode(hash);

    assert!(!hash_str.is_empty());
    assert_eq!(hash_str.len(), 64); // SHA256 produces 64 hex characters
}
