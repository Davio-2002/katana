use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::fs;

pub struct StartArgs {
    pub name: String,
    pub cpp_version: u8,
    pub git: bool,
    pub git_repo: Option<String>,
}

impl StartArgs {
    pub fn prompt() -> Self {
        let theme = ColorfulTheme::default();

        let name = Input::<String>::with_theme(&theme)
            .with_prompt("Project name")
            .interact_text()
            .unwrap();

        let versions = vec!["14", "17", "20", "23"];
        let idx = Select::with_theme(&theme)
            .with_prompt("C++ standard")
            .items(&versions)
            .default(2)
            .interact()
            .unwrap();

        let cpp_version = versions[idx].parse().unwrap();

        let git = Confirm::with_theme(&theme)
            .with_prompt("Init git repo ?")
            .default(false)
            .interact()
            .unwrap();

        let git_repo = if git { Some(name.clone()) } else { None };

        StartArgs {
            name,
            cpp_version,
            git,
            git_repo,
        }
    }
}

pub fn run(args: StartArgs) -> Result<(), String> {
    let n = args.name;
    let version = args.cpp_version;

    let tools = crate::toolcheck::check();

    fs::create_dir_all(format!("{n}/src")).map_err(|e| e.to_string())?;
    fs::create_dir_all(format!("{n}/include")).map_err(|e| e.to_string())?;
    fs::create_dir_all(format!("{n}/.zed")).map_err(|e| e.to_string())?;

    fs::write(
        format!("{n}/CMakeLists.txt"),
        crate::templates::cmake_lists(&n, version),
    )
    .map_err(|e| e.to_string())?;

    fs::write(
        format!("{n}/CMakePresets.json"),
        crate::templates::cmake_presets(),
    )
    .map_err(|e| e.to_string())?;

    fs::write(format!("{n}/.clangd"), crate::templates::clangd(version)).map_err(|e| e.to_string())?;
    fs::write(
        format!("{n}/.clang-format"),
        crate::templates::clang_format(),
    )
    .map_err(|e| e.to_string())?;
    fs::write(format!("{n}/.clang-tidy"), crate::templates::clang_tidy())
        .map_err(|e| e.to_string())?;

    fs::write(format!("{n}/.gitignore"), crate::templates::gitignore())
        .map_err(|e| e.to_string())?;

    fs::write(
        format!("{n}/.zed/settings.json"),
        crate::templates::zed_settings(),
    )
    .map_err(|e| e.to_string())?;

    fs::write(
        format!("{n}/.zed/tasks.json"),
        crate::templates::zed_tasks(&n, tools.watchexec),
    )
    .map_err(|e| e.to_string())?;

    fs::write(format!("{n}/src/main.cpp"), crate::templates::main_cpp(&n))
        .map_err(|e| e.to_string())?;

    fs::write(
        format!("{n}/include/example.h"),
        crate::templates::example_h(),
    )
    .map_err(|e| e.to_string())?;

    fs::write(format!("{n}/README.md"), crate::templates::readme(&n, version))
        .map_err(|e| e.to_string())?;

    if tools.cmake && tools.ninja {
            std::process::Command::new("cmake")
                .args(["--preset", "debug", "-Wno-dev"])
                .current_dir(&n)
                .status()
                .ok();
        }

        if args.git {
            std::process::Command::new("git").arg("init").current_dir(&n).status().ok();
            std::process::Command::new("git").args(["add", "-A"]).current_dir(&n).status().ok();
            std::process::Command::new("git")
                .args(["commit", "-m", &format!("🗡 Initial commit — katana scaffolded {n}")])
                .current_dir(n)
                .status()
                .ok();
        }

    Ok(())
}
