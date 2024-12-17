# Bevy Game Template

This repository provides a project template for quickly starting a new Bevy game in Rust using [cargo-generate](https://github.com/cargo-generate/cargo-generate).

## Overview
This template includes everything you need to get started with the [Bevy Game Engine](https://bevyengine.org/), a cutting-edge, ECS-based game engine built in Rust.

- **Basic Game Structure:** A simple example with a minimal setup for your game
- **Easy-to-Use Template:** Uses `cargo-generate` for quick project initialization

---

## Installation
To use this template, ensure you have the following installed:

- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/)
- **cargo-generate**: A tool for generating Rust project scaffolding

Install `cargo-generate` using:

```bash
cargo install cargo-generate
```

---

## Usage
To create a new project using this template, run the following command:

```bash
cargo generate --git https://github.com/<your-repo>/bevy-template.git --name my_bevy_game
```

Replace `my_bevy_game` with the desired name for your project.

### Running Your Game
Navigate to your newly created project folder:

```bash
cd my_bevy_game
```

Run the project with:

```bash
cargo run
```

---

## License
This template is licensed under the [MIT License](LICENSE).
