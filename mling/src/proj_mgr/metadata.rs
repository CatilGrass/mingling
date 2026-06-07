use serde::Deserialize;
use serde::Serialize;

use std::path::PathBuf;
use std::process::Command;

/// Read cargo metadata by running `cargo metadata` with the given manifest path.
pub fn read_metadata(cargo_toml: &PathBuf) -> Result<CargoLockFile, std::io::Error> {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--manifest-path")
        .arg(cargo_toml)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(std::io::Error::other(format!(
            "cargo metadata failed: {}",
            stderr
        )));
    }

    let lock_file: CargoLockFile = serde_json::from_slice(&output.stdout)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    Ok(lock_file)
}

/// A cargo metadata lock file that serde can serialize and deserialize.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoLockFile {
    pub packages: Vec<Package>,
    #[serde(rename = "workspace_members")]
    pub workspace_members: Vec<String>,
    #[serde(rename = "workspace_default_members")]
    pub workspace_default_members: Vec<String>,
    pub resolve: Resolve,
    #[serde(rename = "target_directory")]
    pub target_directory: String,
    #[serde(rename = "build_directory")]
    pub build_directory: String,
    pub version: u64,
    #[serde(rename = "workspace_root")]
    pub workspace_root: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub id: String,
    pub license: Option<String>,
    #[serde(rename = "license_file")]
    pub license_file: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    pub dependencies: Vec<Dependency>,
    pub targets: Vec<Target>,
    pub features: std::collections::BTreeMap<String, Vec<String>>,
    #[serde(rename = "manifest_path")]
    pub manifest_path: String,
    pub metadata: Option<serde_json::Value>,
    pub publish: Option<serde_json::Value>,
    pub authors: Vec<String>,
    pub categories: Vec<String>,
    pub keywords: Vec<String>,
    pub readme: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub edition: String,
    pub links: Option<String>,
    #[serde(rename = "default_run")]
    pub default_run: Option<String>,
    #[serde(rename = "rust_version")]
    pub rust_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub source: Option<String>,
    pub req: String,
    pub kind: Option<String>,
    pub rename: Option<String>,
    pub optional: bool,
    #[serde(rename = "uses_default_features")]
    pub uses_default_features: bool,
    pub features: Vec<String>,
    pub target: Option<String>,
    pub registry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub kind: Vec<String>,
    #[serde(rename = "crate_types")]
    pub crate_types: Vec<String>,
    pub name: String,
    #[serde(rename = "src_path")]
    pub src_path: String,
    pub edition: String,
    pub doc: bool,
    pub doctest: bool,
    pub test: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "required-features")]
    pub required_features: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolve {
    pub nodes: Vec<ResolveNode>,
    pub root: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveNode {
    pub id: String,
    pub dependencies: Vec<String>,
    pub deps: Vec<DepInfo>,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepInfo {
    pub name: String,
    pub pkg: String,
    #[serde(rename = "dep_kinds")]
    pub dep_kinds: Vec<DepKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepKind {
    pub kind: Option<String>,
    pub target: Option<String>,
}
