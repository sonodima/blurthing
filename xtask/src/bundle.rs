use std::path::Path;

use anyhow::{Context, Result};
use cargo_toml::Package;
use tauri_bundler::{
    AppCategory, BundleBinary, BundleSettings, DmgSettings, PackageSettings, Position,
    SettingsBuilder, Size,
};

use crate::args::BundleArgs;
use crate::utils;

const PRODUCT_NAME: &str = "BlurThing";
const BUNDLE_IDENTIFIER: &str = "com.sonodima.BlurThing";
const COPYRIGHT: &str = "© 2024 Tommaso Dimatore";
const CATEGORY: AppCategory = AppCategory::GraphicsAndDesign;

const APP_ICONS: &str = "icon/*.png";
const DMG_BACKGROUND: &str = "dmg-background.jpg";

pub fn cmd_bundle(args: BundleArgs) -> Result<()> {
    // utils::run_cargo("build --package blurthing --release").unwrap();

    let workspace_dir = utils::get_workspace_dir()?;
    let toml_path = workspace_dir.join("blurthing").join("Cargo.toml");
    let manifest = utils::get_package_manifest(&toml_path)?;
    let binary_name = format!("{}{}", manifest.name(), std::env::consts::EXE_SUFFIX);

    let release_dir = workspace_dir.join("target").join("release");
    anyhow::ensure!(
        release_dir.join(&binary_name).exists(),
        "binary does not exist, did you build the project?"
    );

    let package = package_settings(&manifest)?;
    let bundle = bundle_settings(&workspace_dir);
    let main_binary = BundleBinary::new(binary_name, true);

    let settings = SettingsBuilder::new()
        .package_settings(package)
        .bundle_settings(bundle)
        .binaries(vec![main_binary])
        .project_out_directory(release_dir)
        .build()
        .context("failed to create the bundler settings")?;

    tauri_bundler::bundle_project(settings)
        .map_err(|e| anyhow::anyhow!("failed to bundle the project: {}", e))
        .map(|_| ())
}

fn package_settings(manifest: &Package) -> Result<PackageSettings> {
    let authors = manifest.authors().iter().map(|s| s.to_string()).collect();

    let settings = PackageSettings {
        product_name: PRODUCT_NAME.to_string(),
        version: manifest.version().to_string(),
        description: manifest
            .description()
            .context("description is not set in the package manifest")?
            .to_string(),
        homepage: manifest.homepage().map(|s| s.to_string()),
        authors: Some(authors),
        license: manifest.license().map(|s| s.to_string()),
        default_run: manifest.default_run.as_ref().map(|s| s.to_string()),
    };

    Ok(settings)
}

fn bundle_settings(workspace_dir: &Path) -> BundleSettings {
    let icon = workspace_dir
        .join("assets")
        .join(APP_ICONS)
        .to_string_lossy()
        .to_string();

    let settings = BundleSettings {
        identifier: Some(BUNDLE_IDENTIFIER.to_string()),
        icon: Some(vec![icon]),
        copyright: Some(COPYRIGHT.to_string()),
        category: Some(CATEGORY),
        dmg: dmg_settings(workspace_dir),
        ..Default::default()
    };

    settings
}

fn dmg_settings(workspace_dir: &Path) -> DmgSettings {
    let background = workspace_dir.join("assets").join(DMG_BACKGROUND);

    let settings = DmgSettings {
        background: Some(background),
        window_size: Size {
            width: 700,
            height: 500,
        },
        app_position: Position { x: 170, y: 230 },
        application_folder_position: Position { x: 530, y: 230 },
        ..Default::default()
    };

    settings
}
