//! # sona
//! types from https://www.npmjs.com/package/@kulupu-linku/sona,
//! ported to rust. Up to date as of @kulupu-linku/sona v0.2.0

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub enum Book {
	#[serde(rename = "pu")]
	Pu,
	#[serde(rename = "ku suli")]
	KuSuli,
	#[serde(rename = "ku lili")]
	KuLili,
	#[serde(rename = "none")]
	None,
}

impl From<&str> for Book {
	fn from(s: &str) -> Self {
		match s {
			"pu" => Book::Pu,
			"ku suli" => Book::KuSuli,
			"ku lili" => Book::KuLili,
			_ => Book::None,
		}
	}
}

impl ToString for Book {
	fn to_string(&self) -> String {
		match self {
			Book::Pu => "pu",
			Book::KuSuli => "ku suli",
			Book::KuLili => "ku lili",
			Book::None => "none",
		}
		.into()
	}
}

#[derive(Deserialize, Serialize, Debug)]
pub enum CoinedEra {
	#[serde(rename = "pre-pu")]
	PrePu,
	#[serde(rename = "post-pu")]
	PostPu,
	#[serde(rename = "post-ku")]
	PostKu,
	#[serde(rename = "none")]
	None,
}

impl From<&str> for CoinedEra {
	fn from(s: &str) -> Self {
		match s {
			"pre-pu" => CoinedEra::PrePu,
			"post-pu" => CoinedEra::PostPu,
			"post-ku" => CoinedEra::PostKu,
			_ => CoinedEra::None,
		}
	}
}

impl ToString for CoinedEra {
	fn to_string(&self) -> String {
		match self {
			CoinedEra::PrePu => "pre-pu",
			CoinedEra::PostPu => "post-pu",
			CoinedEra::PostKu => "post-ku",
			CoinedEra::None => "none",
		}
		.into()
	}
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Resources {
	pub sona_pona: Option<String>,
	pub lipamanka_semantic: Option<String>,
}

impl From<HashMap<String, String>> for Resources {
	fn from(h: HashMap<String, String>) -> Self {
		Resources {
			sona_pona: h.get("sona_pona").map(|s| s.to_string()),
			lipamanka_semantic: h.get("lipamanka_semantic").map(|s| s.to_string()),
		}
	}
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Representations {
	pub sitelen_emosi: Option<String>,
	pub sitelen_jelo: Option<Vec<String>>,
	pub ligatures: Option<Vec<String>>,
	pub sitelen_sitelen: Option<String>,
	pub ucsur: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum UsageCategory {
	#[serde(rename = "core")]
	Core,
	#[serde(rename = "common")]
	Common,
	#[serde(rename = "uncommon")]
	Uncommon,
	#[serde(rename = "obscure")]
	Obscure,
	#[serde(rename = "sandbox")]
	Sandbox,
}

impl From<&str> for UsageCategory {
	fn from(s: &str) -> Self {
		match s {
			"core" => UsageCategory::Core,
			"common" => UsageCategory::Common,
			"uncommon" => UsageCategory::Uncommon,
			"obscure" => UsageCategory::Obscure,
			_ => UsageCategory::Sandbox, // usually "sandbox", might be empty string
		}
	}
}

impl ToString for UsageCategory {
	fn to_string(&self) -> String {
		match self {
			UsageCategory::Core => "core",
			UsageCategory::Common => "common",
			UsageCategory::Uncommon => "uncommon",
			UsageCategory::Obscure => "obscure",
			UsageCategory::Sandbox => "sandbox",
		}
		.into()
	}
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Etymology {
	pub word: Option<String>,
	pub alt: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Audio {
	pub link: String,
	pub author: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PuVerbatim {
	pub en: String,
	pub fr: String,
	pub de: String,
	pub eo: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Word {
	pub id: String,
	pub author_verbatim: String,
	pub author_verbatim_source: String,
	pub book: Book,
	pub coined_era: CoinedEra,
	pub coined_year: String,
	pub creator: Vec<String>,
	pub ku_data: Option<HashMap<String, u8>>,
	pub see_also: Vec<String>,
	pub resources: Option<Resources>,
	pub representations: Option<Representations>,
	pub source_language: String,
	pub usage_category: UsageCategory,
	pub word: String,
	pub deprecated: bool,
	pub etymology: Vec<Etymology>,
	pub audio: Vec<Audio>,
	pub pu_verbatim: Option<PuVerbatim>,
	pub usage: HashMap<String, u8>,
}

pub type CommentaryTranslation = HashMap<String, String>;
pub type DefinitionTranslation = HashMap<String, String>;
pub type SitelenPonaTranslation = HashMap<String, String>;

#[derive(Deserialize, Serialize, Debug)]
pub struct InnerEtymologyTranslation {
	pub definition: Option<String>,
	pub language: String,
}

pub type EtymologyTranslation = HashMap<String, InnerEtymologyTranslation>;

#[derive(Deserialize, Serialize, Debug)]
pub struct SignEtymology {
	pub language: String,
	pub sign: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignWriting {
	pub fsw: String,
	pub swu: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Video {
	pub mp4: String,
	pub gif: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sign {
	pub definition: String,
	pub id: String,
	pub is_two_handed: bool,
	pub new_gloss: String,
	pub old_gloss: String,
	pub etymology: Vec<SignEtymology>,
	pub signwriting: SignWriting,
	pub video: Video,
}

pub type FingerspellingSign = Sign;

#[derive(Deserialize, Serialize, Debug)]
pub struct InnerParametersTranslation {
	pub handshape: Option<String>,
	pub movement: Option<String>,
	pub placement: Option<String>,
	pub orientation: Option<String>,
}

pub type ParametersTranslation = HashMap<String, InnerParametersTranslation>;
pub type IconTranslation = HashMap<String, String>;

#[derive(Deserialize, Serialize, Debug)]
pub enum WritingSystem {
	#[serde(rename = "sitelen pona")]
	SitelenPona,
	#[serde(rename = "sitelen sitelen")]
	SitelenSitelen,
	#[serde(rename = "alphabet")]
	Alphabet,
	#[serde(rename = "syllabary")]
	Syllabary,
	#[serde(rename = "logography")]
	Logography,
	#[serde(rename = "tokiponido alphabet")]
	TokiponidoAlphabet,
	#[serde(rename = "tokiponido syllabary")]
	TokiponidoSyllabary,
	#[serde(rename = "tokiponido logography")]
	TokiponidoLogography,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Links {
	fontfile: Option<String>,
	repo: Option<String>,
	webpage: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Font {
	pub id: String,
	pub creator: Vec<String>,
	pub features: Vec<String>,
	pub filename: String,
	pub last_updated: Option<String>,
	pub license: String,
	pub ligatures: bool,
	pub name: String,
	pub style: String,
	pub ucsur: bool,
	pub version: String,
	pub writing_system: WritingSystem,
	pub links: Links,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Direction {
	#[serde(rename = "ltr")]
	Ltr,
	#[serde(rename = "rtl")]
	Rtl,
}

impl From<&str> for Direction {
	fn from(s: &str) -> Self {
		match s {
			"ltr" => Direction::Ltr,
			_ => Direction::Rtl,
		}
	}
}

impl ToString for Direction {
	fn to_string(&self) -> String {
		match self {
			Direction::Ltr => "ltr",
			Direction::Rtl => "rtl",
		}
		.into()
	}
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LangName {
	en: String,
	tok: Option<String>,
	endonym: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Language {
	pub id: String,
	pub locale: String,
	pub direction: Direction,
	pub name: LangName,
}

pub type Languages = HashMap<String, Language>;
