use colored::Colorize;

pub mod gradient;

pub fn banner() {
    let lines = [
        "   ██╗  ██╗ █████╗ ████████╗ █████╗ ███╗   ██╗ █████╗ ",
        "   ██║ ██╔╝██╔══██╗╚══██╔══╝██╔══██╗████╗  ██║██╔══██╗",
        "   █████╔╝ ███████║   ██║   ███████║██╔██╗ ██║███████║",
        "   ██╔═██╗ ██╔══██║   ██║   ██╔══██║██║╚██╗██║██╔══██║",
        "   ██║  ██╗██║  ██║   ██║   ██║  ██║██║ ╚████║██║  ██║",
        "   ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝",
    ];

    println!();
    for (i, line) in lines.iter().enumerate() {
        // each row shifts the gradient slightly — deep at top, bright at bottom
        let offset = i as f32 * 0.08;
        println!("{}", gradient::gradient_line(line, offset, offset + 0.6));
    }
    println!(
        "{}",
        "   C++ project scaffolder for Zed  •  v2.0.0".dimmed()
    );
    println!();
}

pub fn step(msg: &str) {
    println!("\n  {} {}", "⛩️ ".bold(), msg.bold().magenta());
}

pub fn success(msg: &str) {
    println!("  {} {}", "✅", msg);
}

pub fn warn(msg: &str) {
    println!("  {}  {}", "⚠️ ".yellow(), msg.yellow());
}

pub fn error(msg: &str) -> ! {
    eprintln!("  {} {}", "💀", msg.red().bold());
    std::process::exit(1);
}

pub fn info(msg: &str) {
    println!("  \x1b[38;2;0;180;216m🗡\x1b[0m  {}", msg);
}

pub fn dim(msg: &str) {
    println!("{}", format!("    {}", msg).dimmed());
}

pub fn done(name: &str) {
    println!();
    println!(
        "  {}",
        gradient::gradient_line("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━", 0.1, 0.9,)
    );
    println!(
        "  \x1b[1m🗡  {} {}\x1b[0m",
        gradient::gradient_line(name, 0.3, 0.7),
        "is ready!".white().bold()
    );
    println!(
        "  {}",
        gradient::gradient_line("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━", 0.1, 0.9,)
    );
    println!();
}
