extern crate serde;
extern crate serde_json;

use std::path::PathBuf;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;


#[derive(Serialize, Deserialize, Debug)]
#[serde(bound(deserialize = "Metadata: Deserialize<'de>"))]
pub struct CargoMetadata<Metadata = Value> {
	pub packages: Vec<Package<Metadata>>,
	pub target_directory: PathBuf,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(bound(deserialize = "Metadata: Deserialize<'de>"))]
pub struct Package<Metadata> {
	pub name: String,
	pub authors: Vec<String>,
	pub version: String,
	pub description: Option<String>,
	pub manifest_path: String,
	pub targets: Vec<Target>,
	pub metadata: Option<Metadata>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
	pub kind: Vec<String>,
	pub crate_types: Vec<String>,
	pub name: String,
}
