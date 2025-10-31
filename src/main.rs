use std::fs;
use clap::Parser;

// Embed templates at compile time
const BASIC_CARGO_TEMPLATE: &str = include_str!("../templates/Cargo.toml.template");
const BASIC_MAIN_TEMPLATE: &str = include_str!("../templates/main.rs.template");
const AXUM_CARGO_TEMPLATE: &str = include_str!("../templates/axum/Cargo.toml.template");
const AXUM_MAIN_TEMPLATE: &str = include_str!("../templates/axum/main.rs.template");
const GITIGNORE_TEMPLATE: &str = include_str!("../templates/.gitignore.template");

/// Simple Rustverse CLI
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Project name
    name: String,

    /// Template type (e.g. "axum", "basic")
    #[arg(short, long, default_value = "basic")]
    template: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let project_name = &args.name;
    let template = args.template.as_str();

    // สร้างโฟลเดอร์โปรเจกต์
    fs::create_dir_all(format!("{}/src", project_name))?;

    // เลือกเทมเพลตที่เหมาะสม
    let (cargo_template, main_template) = match template {
        "axum" => (AXUM_CARGO_TEMPLATE, AXUM_MAIN_TEMPLATE),
        _ => (BASIC_CARGO_TEMPLATE, BASIC_MAIN_TEMPLATE),
    };

    // สร้าง Cargo.toml โดยแทนที่ placeholder
    let cargo_toml = cargo_template.replace("{{PROJECT_NAME}}", project_name);
    fs::write(format!("{}/Cargo.toml", project_name), cargo_toml)?;

    // สร้าง main.rs
    fs::write(format!("{}/src/main.rs", project_name), main_template)?;

    // สร้าง .gitignore
    fs::write(format!("{}/.gitignore", project_name), GITIGNORE_TEMPLATE)?;

    // สร้าง git repository
    std::process::Command::new("git")
        .args(&["init"])
        .current_dir(project_name)
        .output()
        .ok(); // Ignore errors if git is not installed

    println!("✅ Project '{}' created successfully!", project_name);
    println!("👉 cd {} && cargo run", project_name);
    Ok(())
}
