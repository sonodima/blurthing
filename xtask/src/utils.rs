use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};
use cargo_toml::{Manifest, Package};

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn get_workspace_dir() -> Result<PathBuf> {
    Path::new(CARGO_MANIFEST_DIR)
        .ancestors()
        .nth(1)
        .context("workspace directory not found")
        .map(|p| p.to_path_buf())
}

pub fn get_package_manifest(toml_path: &Path) -> Result<Package> {
    Manifest::from_path(toml_path)
        .map_err(|e| anyhow::anyhow!("failed to parse the package manifest: {}", e))?
        .package
        .context("the target file is not a package manifest")
}

pub fn run_cargo(args: &str) -> Result<()> {
    if Command::new("cargo")
        .args(args.trim().split_whitespace())
        .status()?
        .success()
    {
        Ok(())
    } else {
        Err(anyhow::anyhow!("cargo command failed"))
    }
}
