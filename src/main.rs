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

    fs::create_dir_all(format!("{}/src", project_name))?;

    let cargo_toml = match template {
        "axum" => format!(
            "[package]
name = \"{}\"
version = \"0.1.0\"
edition = \"2024\"

[dependencies]
axum = \"0.6.9\"
tokio = {{ version = \"1\", features = [\"full\"] }}
",
            project_name
        ),
        _ => format!(
            "[package]
name = \"{}\"
version = \"0.1.0\"
edition = \"2024\"

[dependencies]
",
            project_name
        ),
    };

    fs::write(format!("{}/Cargo.toml", project_name), cargo_toml)?;

    let main_rs = match template {
        "axum" => r#"use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello Rustverse (Axum)!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
"#,
        _ => r#"fn main() {
    println!("Hello Rustverse!");
}
"#,
    };

    fs::write(format!("{}/src/main.rs", project_name), main_rs)?;

    println!("✅ Project '{}' created successfully!", project_name);
    println!("👉 cd {} && cargo run", project_name);
    Ok(())
}
