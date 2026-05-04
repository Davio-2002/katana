pub struct StartArgs {
    pub name: String,
    pub cpp_version: u8,
    pub git: bool,
    pub git_repo: Option<String>,
}

pub fn run(args: StartArgs) /*-> Result<(), String>*/ {}
