use chrono::{Datelike, NaiveDate};
use id3::frame::{Picture, PictureType, Timestamp};
use id3::{Tag, Version, TagLike};
use std::path::{Path, PathBuf};

use crate::error::SpotifyError;

use super::Field;

pub struct ID3Tag {
	path: PathBuf,
	tag: Tag,
	separator: String,
	version: Version,
}

impl ID3Tag {
	/// Load form path
	pub fn open(path: impl AsRef<Path>) -> Result<ID3Tag, SpotifyError> {
		let tag = Tag::read_from_path(&path).unwrap_or_default();

		Ok(ID3Tag {
			path: path.as_ref().to_owned(),
			tag,
			separator: String::new(),
			version: Version::Id3v23,
		})
	}

	/// Wether to use ID3v2.4
	pub fn use_id3_v24(&mut self, v: bool) {
		match v {
			true => self.version = Version::Id3v24,
			false => self.version = Version::Id3v23,
		}
	}
}

impl super::Tag for ID3Tag {
	fn set_separator(&mut self, separator: &str) {
		self.separator = separator.to_string();
	}

	fn set_raw(&mut self, tag: &str, value: Vec<String>) {
		self.tag.set_text(tag, value.join(&self.separator));
	}

	fn set_field(&mut self, field: Field, value: Vec<String>) {
		let tag = match field {
			Field::Title => "TIT2",
			Field::Artist => "TPE1",
			Field::Album => "TALB",
			Field::TrackNumber => "TPOS",
			Field::DiscNumber => "TRCK",
			Field::Genre => "TCON",
			Field::Label => "TPUB",
			Field::AlbumArtist => "TPE2",
		};
		self.set_raw(tag, value);
	}

	fn save(&mut self) -> Result<(), SpotifyError> {
		Ok(self.tag.write_to_path(&self.path, self.version)?)
	}

	fn add_cover(&mut self, mime: &str, data: Vec<u8>) {
		self.tag.add_frame(Picture {
			mime_type: mime.to_owned(),
			picture_type: PictureType::CoverFront,
			description: "cover".to_string(),
			data,
		});
	}

	fn set_release_date(&mut self, date: NaiveDate) {
		self.tag.set_date_released(Timestamp {
			year: date.year(),
			month: Some(date.month() as u8),
			day: Some(date.day() as u8),
			hour: None,
			minute: None,
			second: None,
		})
	}
}
