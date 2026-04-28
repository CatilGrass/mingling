use std::path::PathBuf;

use serde::Serialize;

pub type BinaryName = String;
pub type BinaryTargetPath = PathBuf;

pub struct ProjectSolveResult {
    pub target_dir: PathBuf,
    pub workspace_root: PathBuf,
    pub binaries: Vec<BinaryItem>,
}

#[derive(Debug, Serialize)]
pub struct BinaryItem {
    pub name: String,
    pub path: PathBuf,
}

pub fn solve_current_dir() -> Result<ProjectSolveResult, std::io::Error> {
    let current = std::env::current_dir()?;
    solve(current)
}

pub fn solve(current: PathBuf) -> Result<ProjectSolveResult, std::io::Error> {
    let (target_dir, workspace_root, binaries) = solve_inner(&current)?;
    Ok(ProjectSolveResult {
        target_dir,
        workspace_root,
        binaries,
    })
}

fn solve_inner(current: &PathBuf) -> Result<(PathBuf, PathBuf, Vec<BinaryItem>), std::io::Error> {
    let output = std::process::Command::new("cargo")
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .current_dir(current)
        .output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("cargo metadata failed: {}", stderr),
        ));
    }
    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let workspace_root_str = metadata["workspace_root"].as_str().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "missing workspace_root")
    })?;
    let workspace_root = PathBuf::from(workspace_root_str);

    let target_dir_str = metadata["target_directory"].as_str().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "missing target_directory")
    })?;
    let target_dir = PathBuf::from(target_dir_str);

    let packages = metadata["packages"].as_array().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "missing packages array")
    })?;

    let mut binaries = Vec::new();
    let cargo_toml_path = workspace_root.join("Cargo.toml");

    // Find the package whose manifest_path matches workspace_root/Cargo.toml
    for pkg in packages {
        let manifest_path = pkg["manifest_path"].as_str().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "missing manifest_path")
        })?;
        let manifest_path_buf = PathBuf::from(manifest_path);
        if manifest_path_buf == cargo_toml_path {
            // Found the workspace root package
            if let Some(targets) = pkg["targets"].as_array() {
                for target in targets {
                    let kind = target["kind"].as_array();
                    let is_bin = kind
                        .map(|k| k.iter().any(|v| v.as_str() == Some("bin")))
                        .unwrap_or(false);
                    if is_bin {
                        let name = target["name"].as_str().ok_or_else(|| {
                            std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "missing target name",
                            )
                        })?;
                        let mut binary_path = target_dir.join("release").join(name);
                        if cfg!(target_os = "windows") {
                            binary_path.set_extension("exe");
                        }
                        binaries.push(BinaryItem {
                            name: name.to_string(),
                            path: binary_path,
                        });
                    }
                }
            }
            break;
        }
    }

    Ok((target_dir, workspace_root, binaries))
}
