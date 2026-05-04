pub mod commands;
pub mod templates;
pub mod toolcheck;
pub mod ui;

use dialoguer::{Select, theme::ColorfulTheme};

fn main() {
    ui::banner();
    let items = vec!["⚔️  start", "📦  import"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("  🗡  katana")
        .items(&items)
        .default(0)
        .interact()
        .unwrap();
    match selection {
        0 => { let args = commands::start::StartArgs::prompt(); commands::start::run(args).unwrap(); }
        1 => {println!("Not yet")}
        _ => unreachable!(),
    }
}
