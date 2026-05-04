pub mod commands;
pub mod templates;
pub mod toolcheck;
pub mod ui;

use dialoguer::{Select, theme::ColorfulTheme};

fn main() {
    ui::banner();

    let items = vec!["⚔️  start", "📦  import"];

    let theme = ColorfulTheme::default();

    let selection = Select::with_theme(&theme as &dyn dialoguer::theme::Theme)
        .with_prompt("  🗡  katana")
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => println!("start"),
        1 => println!("import"),
        _ => unreachable!(),
    }
}
