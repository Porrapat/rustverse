use std::fs;
use clap::Parser;

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

    // อ่านเทมเพลต Cargo.toml
    let cargo_template_path = match template {
        "axum" => "templates/axum/Cargo.toml.template",
        _ => "templates/Cargo.toml.template",
    };

    let cargo_toml = fs::read_to_string(cargo_template_path)
        .map_err(|e| std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Template file '{}' not found: {}", cargo_template_path, e)
        ))?
        .replace("{{PROJECT_NAME}}", project_name);

    fs::write(format!("{}/Cargo.toml", project_name), cargo_toml)?;

    // อ่านเทมเพลต main.rs
    let main_template_path = match template {
        "axum" => "templates/axum/main.rs.template",
        _ => "templates/main.rs.template",
    };

    let main_rs = fs::read_to_string(main_template_path)
        .map_err(|e| std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Template file '{}' not found: {}", main_template_path, e)
        ))?;

    fs::write(format!("{}/src/main.rs", project_name), main_rs)?;

    println!("✅ Project '{}' created successfully!", project_name);
    println!("👉 cd {} && cargo run", project_name);
    Ok(())
}
