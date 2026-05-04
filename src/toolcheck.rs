#[derive(Debug)]
pub struct ToolStatus {
    pub cmake: bool,
    pub ninja: bool,
    pub clangd: bool,
    pub watchexec: bool,
}

fn probe(tool: &str) -> bool {
    let cmd = if cfg!(target_os = "windows") {
        "where"
    } else {
        "which"
    };
    std::process::Command::new(cmd)
        .arg(tool)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn check() -> ToolStatus {
    ToolStatus {
        cmake: probe("cmake"),
        ninja: probe("ninja"),
        clangd: probe("clangd"),
        watchexec: probe("watchexec"),
    }
}
