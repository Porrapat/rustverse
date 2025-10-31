use std::fs;
use clap::Parser;

/// Simple Rustverse CLI
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Project name
    name: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let project_name = &args.name;

    // สร้างโฟลเดอร์โปรเจกต์
    fs::create_dir_all(format!("{}/src", project_name))?;

    // เขียน Cargo.toml
    let cargo_toml = format!(
        "[package]
name = \"{}\"
version = \"0.1.0\"
edition = \"2024\"

[dependencies]
",
        project_name
    );

    fs::write(format!("{}/Cargo.toml", project_name), cargo_toml)?;

    // เขียน main.rs
    let main_rs = r#"fn main() {
    println!("Hello Rustverse!");
}
"#;
    fs::write(format!("{}/src/main.rs", project_name), main_rs)?;

    println!("✅ Project '{}' created successfully!", project_name);
    println!("👉 cd {} && cargo run", project_name);
    Ok(())
}
