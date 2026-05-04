pub struct ToolStatus {
    pub cmake: bool,
    pub ninja: bool,
    pub clangd: bool,
    pub watchexec: bool,
}

pub fn check() /* -> ToolStatus */ {}
