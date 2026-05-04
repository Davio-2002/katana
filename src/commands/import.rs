use dialoguer::{Confirm, Input, theme::ColorfulTheme};

pub struct ImportArgs {
    pub path: String,
    pub force: bool,
}

impl ImportArgs {
    pub fn prompt() -> Self {
        let theme = ColorfulTheme::default();

        let path: String = Input::<String>::with_theme(&theme)
            .with_prompt("Project path")
            .default(".".to_string())
            .interact_text()
            .unwrap();

        let force = Confirm::with_theme(&theme)
            .with_prompt("Force overwrite existing configs?")
            .default(false)
            .interact()
            .unwrap();

        ImportArgs { path, force }
    }
}

use std::fs;
use std::path::Path;

pub fn run(args: ImportArgs) -> Result<(), String> {
    let p = Path::new(&args.path);

    if !p.exists() {
        return Err(format!("Directory not found: {}", args.path));
    }
    if !p.join("CMakeLists.txt").exists() {
        return Err(format!("No CMakeLists.txt found in {}", args.path));
    }

    let cmake_content = fs::read_to_string(p.join("CMakeLists.txt")).map_err(|e| e.to_string())?;

    let name = cmake_content
        .lines()
        .find(|l| l.to_lowercase().contains("project("))
        .and_then(|l| l.split('(').nth(1))
        .and_then(|l| l.split_whitespace().next())
        .unwrap_or("project")
        .to_string();

    let version: u8 = cmake_content
        .lines()
        .find(|l| l.contains("CMAKE_CXX_STANDARD") && !l.contains("REQUIRED"))
        .and_then(|l| l.split_whitespace().last())
        .and_then(|v| v.trim_end_matches(')').parse().ok())
        .unwrap_or(20);

    let tools = crate::toolcheck::check();


    if !cmake_content.contains("CMAKE_EXPORT_COMPILE_COMMANDS") {
        let patched = cmake_content + "\nset(CMAKE_EXPORT_COMPILE_COMMANDS ON)\n";
        fs::write(p.join("CMakeLists.txt"), patched).map_err(|e| e.to_string())?;
    }

    if !p.join("CMakePresets.json").exists() {
        fs::write(p.join("CMakePresets.json"), crate::templates::cmake_presets())
            .map_err(|e| e.to_string())?;
    }

    let write_if = |filename: &str, content: String| -> Result<(), String> {
        let target = p.join(filename);
        if !target.exists() || args.force {
            fs::write(target, content).map_err(|e| e.to_string())?;
        }
        Ok(())
    };

    write_if(".clangd",       crate::templates::clangd(version))?;
    write_if(".clang-format", crate::templates::clang_format().to_string())?;
    write_if(".clang-tidy",   crate::templates::clang_tidy().to_string())?;

    let zed = p.join(".zed");
    if !zed.exists() || args.force {
        fs::create_dir_all(&zed).map_err(|e| e.to_string())?;
        fs::write(zed.join("settings.json"), crate::templates::zed_settings())
            .map_err(|e| e.to_string())?;
        fs::write(zed.join("tasks.json"), crate::templates::zed_tasks(&name, tools.watchexec))
            .map_err(|e| e.to_string())?;
    }

    if tools.cmake && tools.ninja {
        std::process::Command::new("cmake")
            .args(["--preset", "debug", "-Wno-dev"])
            .current_dir(p)
            .status()
            .ok();
    }

    let cc = p.join("build/debug/compile_commands.json");
    let cc_root = p.join("compile_commands.json");
    if cc.exists() && !cc_root.exists() {
        #[cfg(unix)]
        std::os::unix::fs::symlink(&cc, &cc_root).ok();
    }

    println!("  🗡  {} is ready in Zed!", name);
    Ok(())
}
