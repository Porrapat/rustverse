# 🦀 Rustverse

[![Crates.io](https://img.shields.io/crates/v/rustverse.svg)](https://crates.io/crates/rustverse)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Build Status](https://img.shields.io/badge/status-stable-brightgreen.svg)](#)

A friendly project generator for Rust — like `cargo new`, but more fun ✨  
Quickly scaffold projects such as **Axum 0.6.9**, CLI tools, and more.

---

## 🚀 Installation

You can install **Rustverse** from [crates.io](https://crates.io/crates/rustverse):

```bash
cargo install rustverse
```

Then run it anywhere on your system:

```bash
rustverse my_app
```

---

## 🧩 Templates

| Template | Description |
|-----------|-------------|
| `basic`   | A simple “Hello Rustverse!” CLI |
| `axum`    | Axum 0.6.9 web server template |

---

## 💻 Usage

### 🟢 Create a basic project
```bash
rustverse my_basic
```

Then run:
```bash
cd my_basic
cargo run
```

Output:
```
Hello Rustverse!
```

---

### 🟣 Create an Axum web project
```bash
rustverse my_axum --template axum
```

Then run:
```bash
cd my_axum
cargo run
```

Server output:
```
🚀 Server running at http://127.0.0.1:3000
```

Open your browser and visit [http://localhost:3000](http://localhost:3000)

---

## 🛠 Development

Clone the repository and build from source:

```bash
git clone https://github.com/Porrapat/rustverse.git
cd rustverse
cargo build
```

Run locally:
```bash
cargo run -- my_test
```

---

## 🪪 License

Licensed under the [MIT License](./LICENSE).

---

> Made with ❤️ by **Porrapat Petchdamrongskul**  
> “Hello Rustverse — where your journey begins.”
