use std::{env::consts::EXE_SUFFIX, path::{Path, PathBuf}};

use anyhow::{anyhow, ensure, Context, Result};
use cargo_toml::{Manifest, Package};
use log::Level;
use tauri_bundler::{BundleBinary, BundleSettings, DmgSettings, MacOsSettings, PackageSettings, Position, SettingsBuilder, Size};

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

const TARGET_CRATE: &str = "blurthing";
const PRODUCT_NAME: &str = "BlurThing";

fn package_settings(product_name: &str, package_manifest: &Package) -> Result<PackageSettings> {
    let authors = package_manifest
        .authors()
        .iter()
        .map(|s| s.to_string())
        .collect();
    let settings = PackageSettings {
        product_name: product_name.to_string(),
        version: package_manifest.version().to_string(),
        description: package_manifest
            .description()
            .context("description is not set in the package manifest")?
            .to_string(),
        homepage: package_manifest.homepage().map(|s| s.to_string()),
        authors: Some(authors),
        license: package_manifest.license().map(|s| s.to_string()),
        default_run: package_manifest.default_run.as_ref().map(|s| s.to_string()),
    };

    Ok(settings)
}

fn dmg_settings(workspace_dir: PathBuf) -> DmgSettings {
    let background = workspace_dir.join("assets").join("dmg-background.jpg");

    DmgSettings {
        background: Some(background),
        window_size: Size {
            width: 700,
            height: 500,
        },
        app_position: Position {
            x: 170,
            y: 230,
        },
        application_folder_position: Position {
            x: 540,
            y: 230,
        },
        ..Default::default()
    }
}

fn bundle_settings(workspace_dir: PathBuf) -> Result<BundleSettings> {
    let settings = BundleSettings {
        identifier: Some("com.example.blurthing".to_string()),
        // icon
        // copyright
        // category
        dmg: dmg_settings(workspace_dir),
        ..Default::default()
    };

    Ok(settings)
}

fn get_package_manifest(toml_path: &Path) -> Result<Package> {
    Manifest::from_path(toml_path)
        .context("failed to parse the manifest file")?
        .package
        .context("the target file is not a package manifest")
}

fn make_bundle() -> Result<()> {
    let workspace_dir = Path::new(CARGO_MANIFEST_DIR)
        .ancestors()
        .nth(1)
        .context("workspace directory not found")?;

    let toml_path = workspace_dir.join(TARGET_CRATE).join("Cargo.toml");
    let package_manifest = get_package_manifest(&toml_path)?;
    let package_settings = package_settings(PRODUCT_NAME, &package_manifest)?;
    let bundle_settings = bundle_settings(workspace_dir.to_owned())?;

    let binary_name = format!("blurthing{}", EXE_SUFFIX);
    let release_dir = workspace_dir.join("target").join("release");
    let binary_path = release_dir.join(&binary_name);
    ensure!(
        binary_path.exists() && binary_path.metadata().map(|m| m.len() > 0).unwrap_or(false),
        "binary does not exist or is empty, did you compile the project?"
    );

    let main_binary = BundleBinary::new(binary_name, true);
    let settings = SettingsBuilder::new()
        .package_settings(package_settings)
        .bundle_settings(bundle_settings)
        .binaries(vec![main_binary])
        .project_out_directory(release_dir)
        // .target()
        // .package_types()
        .log_level(Level::Trace)
        .build()
        .context("failed to create the bundler the settings")?;

    tauri_bundler::bundle_project(settings)
        .map_err(|e| anyhow!("failed to bundle the project: {}", e))?;
    Ok(())
}

fn main() {
    if let Err(e) = make_bundle() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
