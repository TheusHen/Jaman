use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs::{self, File};
use std::io::Write;
use futures_util::StreamExt;
use tempfile::TempDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableVersion {
    pub version: String,
    pub vendor: String,
    pub is_lts: bool,
    pub architecture: String,
    pub download_url: String,
    pub checksum: Option<String>,
}

pub struct Downloader {
    client: Client,
}

impl Downloader {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("jaman/0.1.0")
                .build()
                .unwrap(),
        }
    }

    /// Fetch available Java versions from Adoptium API
    pub async fn fetch_available_versions(&self) -> Result<Vec<AvailableVersion>> {
        let mut versions = Vec::new();

        // Fetch from Adoptium (Eclipse Temurin)
        versions.extend(self.fetch_adoptium_versions().await?);

        Ok(versions)
    }

    async fn fetch_adoptium_versions(&self) -> Result<Vec<AvailableVersion>> {
        let mut versions = Vec::new();
        let base_url = "https://api.adoptium.net/v3/info/available_releases";

        let response: AdoptiumAvailableReleases = self.client
            .get(base_url)
            .send()
            .await?
            .json()
            .await?;

        let os = if cfg!(windows) {
            "windows"
        } else if cfg!(target_os = "macos") {
            "mac"
        } else {
            "linux"
        };

        let arch = match std::env::consts::ARCH {
            "x86_64" => "x64",
            "aarch64" => "aarch64",
            _ => "x64",
        };

        // Fetch details for each version
        for version in response.available_releases.iter().take(10) {
            let url = format!(
                "https://api.adoptium.net/v3/assets/latest/{}/hotspot?architecture={}&image_type=jdk&os={}",
                version, arch, os
            );

            if let Ok(response) = self.client.get(&url).send().await {
                if let Ok(assets) = response.json::<Vec<AdoptiumAsset>>().await {
                    for asset in assets {
                        let download_url = asset.binary.package.link;
                        let checksum = asset.binary.package.checksum;

                        versions.push(AvailableVersion {
                            version: asset.version.semver,
                            vendor: "Eclipse Temurin".to_string(),
                            is_lts: response.available_lts_releases.contains(version),
                            architecture: arch.to_string(),
                            download_url,
                            checksum: Some(checksum),
                        });
                    }
                }
            }
        }

        Ok(versions)
    }

    /// Download and install a Java version
    pub async fn download_and_install(
        &self,
        version: &AvailableVersion,
        installation_dir: &PathBuf,
    ) -> Result<PathBuf> {
        let temp_dir = TempDir::new()?;
        let filename = self.extract_filename(&version.download_url);
        let temp_file = temp_dir.path().join(&filename);

        // Download with progress bar
        self.download_file(&version.download_url, &temp_file).await?;

        // Verify checksum if available
        if let Some(ref checksum) = version.checksum {
            self.verify_checksum(&temp_file, checksum)?;
        }

        // Extract archive
        let extract_dir = installation_dir.join(&format!("{}-{}", version.vendor.replace(" ", "_"), version.version));
        fs::create_dir_all(&extract_dir)?;

        self.extract_archive(&temp_file, &extract_dir)?;

        // Find the actual JDK directory (might be nested)
        let jdk_dir = self.find_jdk_root(&extract_dir)?;

        Ok(jdk_dir)
    }

    async fn download_file(&self, url: &str, dest: &PathBuf) -> Result<()> {
        let response = self.client.get(url).send().await?;
        let total_size = response.content_length().unwrap_or(0);

        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
                .progress_chars("#>-"),
        );

        let mut file = File::create(dest)?;
        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk)?;
            downloaded += chunk.len() as u64;
            pb.set_position(downloaded);
        }

        pb.finish_with_message("Download complete");
        Ok(())
    }

    fn verify_checksum(&self, file: &PathBuf, expected: &str) -> Result<()> {
        use sha2::{Sha256, Digest};

        let mut file = File::open(file)?;
        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher)?;
        let hash = hasher.finalize();
        let hash_str = hex::encode(hash);

        if hash_str.to_lowercase() != expected.to_lowercase() {
            anyhow::bail!("Checksum verification failed");
        }

        Ok(())
    }

    fn extract_archive(&self, archive: &PathBuf, dest: &PathBuf) -> Result<()> {
        let pb = ProgressBar::new_spinner();
        pb.set_message("Extracting archive...");
        pb.enable_steady_tick(std::time::Duration::from_millis(100));

        let extension = archive.extension().and_then(|s| s.to_str()).unwrap_or("");

        match extension {
            "zip" => self.extract_zip(archive, dest)?,
            "gz" => self.extract_tar_gz(archive, dest)?,
            _ => anyhow::bail!("Unsupported archive format: {}", extension),
        }

        pb.finish_with_message("Extraction complete");
        Ok(())
    }

    fn extract_zip(&self, archive: &PathBuf, dest: &PathBuf) -> Result<()> {
        let file = File::open(archive)?;
        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = dest.join(file.name());

            if file.is_dir() {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    fs::create_dir_all(p)?;
                }
                let mut outfile = File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }

            // Set permissions on Unix
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
                }
            }
        }

        Ok(())
    }

    fn extract_tar_gz(&self, archive: &PathBuf, dest: &PathBuf) -> Result<()> {
        let tar_gz = File::open(archive)?;
        let tar = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = tar::Archive::new(tar);
        archive.unpack(dest)?;
        Ok(())
    }

    fn find_jdk_root(&self, extract_dir: &PathBuf) -> Result<PathBuf> {
        // Sometimes archives have a top-level directory
        for entry in fs::read_dir(extract_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                let bin_dir = path.join("bin");
                let java_exe = if cfg!(windows) {
                    bin_dir.join("java.exe")
                } else {
                    bin_dir.join("java")
                };

                if java_exe.exists() {
                    return Ok(path);
                }
            }
        }

        // If not found, assume extract_dir is the root
        Ok(extract_dir.clone())
    }

    fn extract_filename(&self, url: &str) -> String {
        url.split('/')
            .last()
            .unwrap_or("download.zip")
            .to_string()
    }
}

// Adoptium API structures
#[derive(Debug, Deserialize)]
struct AdoptiumAvailableReleases {
    available_releases: Vec<u32>,
    available_lts_releases: Vec<u32>,
}

#[derive(Debug, Deserialize)]
struct AdoptiumAsset {
    binary: AdoptiumBinary,
    version: AdoptiumVersion,
}

#[derive(Debug, Deserialize)]
struct AdoptiumBinary {
    package: AdoptiumPackage,
}

#[derive(Debug, Deserialize)]
struct AdoptiumPackage {
    link: String,
    checksum: String,
}

#[derive(Debug, Deserialize)]
struct AdoptiumVersion {
    semver: String,
}
