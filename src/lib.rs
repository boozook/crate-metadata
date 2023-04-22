#![cfg(not(doctest))]
/*!
	__For usage from build-script.__

	Utility functions that returns current crate metadata
	as result of call `cargo metadata`.
*/

use std::env;
use std::path::{Path, PathBuf};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::process::Command;

pub use model::*;
mod model;

type Error = Box<dyn std::error::Error>;
type Result<T = (), E = self::Error> = std::result::Result<T, E>;

const CARGO_MANIFEST: &str = "Cargo.toml";
const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";
const CARGO_PKG_NAME: &str = "CARGO_PKG_NAME";


/// Cargo metadata for caller crate.
/// Without other packages if it's in a workspace.
/// Caller crate means crate that currently building.
pub fn crate_metadata<Metadata>() -> Result<CargoMetadata<Metadata>>
	where for<'de> Metadata: serde::de::Deserialize<'de> {
	let path =
		PathBuf::from(env::var_os(CARGO_MANIFEST_DIR).ok_or(IoError::new(IoErrorKind::NotFound, CARGO_MANIFEST_DIR))?).join(CARGO_MANIFEST);
	let name = env::var(CARGO_PKG_NAME).unwrap();
	crate_metadata_for(path, &name)
}

/// Cargo metadata for caller crate with other packages that in workspace.
/// Caller crate means crate that currently building.
pub fn cargo_metadata<Metadata>() -> Result<CargoMetadata<Metadata>>
	where for<'de> Metadata: serde::de::Deserialize<'de> {
	let path =
		PathBuf::from(env::var_os(CARGO_MANIFEST_DIR).ok_or(IoError::new(IoErrorKind::NotFound, CARGO_MANIFEST_DIR))?).join(CARGO_MANIFEST);
	cargo_metadata_for(path)
}


/// Cargo metadata for specified manifest path,
/// filtered for specified crate name.
/// That means that there is only one packages should be, with name end path eq specified.
///
/// ```
/// let path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml");
/// let name = env::var("CARGO_PKG_NAME").unwrap();
/// let metadata = meta::crate_metadata_for::<Metadata, _>(&path, &name)?;
/// ```
pub fn crate_metadata_for<Metadata, P: AsRef<Path>>(manifest: P, name: &str) -> Result<CargoMetadata<Metadata>>
	where for<'de> Metadata: serde::de::Deserialize<'de> {
	let manifest_path_str = manifest.as_ref()
	                                .to_str()
	                                .ok_or(IoError::new(IoErrorKind::InvalidInput, CARGO_MANIFEST))?;

	let mut metadata: CargoMetadata<Metadata> = cargo_metadata_for(&manifest)?;
	metadata.packages = metadata.packages
	                            .into_iter()
	                            .filter(|p| p.name == name && p.manifest_path == manifest_path_str)
	                            .collect();

	Ok(metadata)
}


/// Cargo metadata for specified manifest path.
///
/// ```
/// let path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml");
/// let metadata = meta::cargo_metadata_for::<Metadata, _>(&path)?;
/// ```
pub fn cargo_metadata_for<Metadata, P: AsRef<Path>>(manifest: P) -> Result<CargoMetadata<Metadata>>
	where for<'de> Metadata: serde::de::Deserialize<'de> {
	let manifest_path_str = manifest.as_ref()
	                                .to_str()
	                                .ok_or(IoError::new(IoErrorKind::InvalidInput, CARGO_MANIFEST))?;
	let args = [
	            "metadata",
	            "--offline",
	            "--locked",
	            "--frozen",
	            "--no-deps",
	            "--format-version=1",
	            "--manifest-path",
	            manifest_path_str,
	];

	let cargo = env::var("CARGO").ok().unwrap_or("cargo".to_string());
	let metadata_json_raw = Command::new(cargo).args(&args).output()?;
	let stdout = std::str::from_utf8(&metadata_json_raw.stdout)?;

	serde_json::from_str(stdout).map_err(Into::into)
}
