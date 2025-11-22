// Library exports for testing and potential library usage

pub mod config;
pub mod detector;
pub mod downloader;
pub mod path_manager;

// Re-export commonly used types
pub use config::{Config, JavaVersion};
pub use detector::JavaDetector;
pub use downloader::{AvailableVersion, Downloader};
pub use path_manager::PathManager;
