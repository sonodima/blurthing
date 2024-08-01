use std::path::Path;
use std::vec;

use anyhow::{Context, Result};
use cargo_toml::Package;
use tauri_bundler::{
    AppCategory, BundleBinary, BundleSettings, DmgSettings, PackageSettings, PackageType, Position,
    SettingsBuilder, Size, WindowsSettings,
};

use crate::args::BundleArgs;
use crate::utils;

const TARGET_PACKAGE: &str = "blurthing";

const PRODUCT_NAME: &str = "BlurThing";
const BUNDLE_IDENTIFIER: &str = "com.sonodima.BlurThing";
const COPYRIGHT: &str = "© 2024 Tommaso Dimatore";
const CATEGORY: AppCategory = AppCategory::GraphicsAndDesign;

const APP_ICONS: &str = "icon/*";
const WINDOWS_ICON: &str = "icon/icon.ico";
const DMG_BACKGROUND: &str = "dmg-background.jpg";

pub fn cmd_bundle(args: BundleArgs) -> Result<()> {
    let workspace_dir = utils::get_workspace_dir()?;
    let toml_path = workspace_dir.join(TARGET_PACKAGE).join("Cargo.toml");

    let manifest = utils::get_package_manifest(&toml_path)?;
    compile_package(manifest.name().to_string(), args.release, &args.target)?;

    let binary_suffix = utils::get_binary_suffix(&args.target);
    let binary_name = format!("{}{}", manifest.name(), binary_suffix);
    let main_binary = BundleBinary::new(binary_name, true);

    let target_dir = utils::get_target_dir(&workspace_dir, &args.target, args.release);
    let mut settings_builder = SettingsBuilder::new()
        .package_settings(package_settings(&manifest)?)
        .bundle_settings(bundle_settings(&workspace_dir))
        .package_types(package_types(&args.target))
        .binaries(vec![main_binary])
        .project_out_directory(target_dir);

    if let Some(target) = args.target {
        settings_builder = settings_builder.target(target);
    }

    let settings = settings_builder
        .build()
        .context("failed to create the bundler settings")?;

    tauri_bundler::bundle_project(settings)
        .map_err(|e| anyhow::anyhow!("failed to bundle the project: {}", e))
        .map(|_| ())
}

fn compile_package(package: String, release: bool, target: &Option<String>) -> Result<()> {
    let mut build_args = vec!["build".to_string(), "--bin".to_string(), package];

    if release {
        build_args.push("--release".to_string());
    }

    if let Some(target) = target {
        build_args.push("--target".to_string());
        build_args.push(target.to_string());
    }

    utils::run_cargo(&build_args)
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
        windows: windows_settings(workspace_dir),
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

fn windows_settings(workspace_dir: &Path) -> WindowsSettings {
    let icon_path = workspace_dir.join("assets").join(WINDOWS_ICON);

    let settings = WindowsSettings {
        icon_path,
        ..Default::default()
    };

    settings
}

fn package_types(target: &Option<String>) -> Vec<PackageType> {
    match utils::get_target_os(target).as_str() {
        "macos" => vec![PackageType::Dmg],
        "windows" => vec![PackageType::Nsis],
        "linux" => vec![PackageType::Deb, PackageType::Rpm],
        _ => vec![],
    }
}
