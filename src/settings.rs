use crate::downloader::DownloaderConfig;
use crate::error::SpotifyError;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::path::Path;

use tokio::{
	fs::File,
	io::{AsyncReadExt, AsyncWriteExt},
};

// Structure for holding all the settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
	pub username: String,
	pub password: String,
	pub client_id: String,
	pub client_secret: String,
	pub refresh_ui_seconds: u64,
	pub downloader: DownloaderConfig,
}

impl Settings {
	// Create new instance
	pub fn new(username: &str, password: &str, client_id: &str, client_secret: &str) -> Settings {
		Settings {
			username: username.to_string(),
			password: password.to_string(),
			client_id: client_id.to_string(),
			client_secret: client_secret.to_string(),
			refresh_ui_seconds: 1,
			downloader: DownloaderConfig::new(),
		}
	}

	// Serialize the settings to a json file
	pub async fn save(&self) -> Result<(), SpotifyError> {
		let data = serde_json::to_string_pretty(self)?;
		let mut file = File::create(Path::new("settings.json")).await?;
		file.write_all(data.as_bytes()).await?;
		Ok(())
	}

	// Deserialize the settings from a json file
	pub async fn load(raw_path: &str) -> Result<Settings, SpotifyError> {
		// default locations
		let settings_paths: [&Path; 2] = [
			Path::new("settings.json"),
			&home_dir().unwrap().join(".config/downonspot.json"),
		];

		let mut path = Path::new(raw_path);
		if raw_path == "" {
			for p in settings_paths {
				if !p.exists() {
					continue;
				}

				path = p;
				break;
			}
		}

		let mut file = File::open(path).await?;
		let mut buf = String::new();
		file.read_to_string(&mut buf).await?;
		Ok(serde_json::from_str(&buf)?)
	}
}
