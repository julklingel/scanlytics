# Scanlytics

Scanlytics is a Tauri application built with SvelteKit and TypeScript. This project uses Vite for development and build processes, and integrates with a Rust backend using Tauri.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/)
- [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Project Structure




## Prerequisites

- [Node.js](https://nodejs.org/) (version 16 or higher)
- [pnpm](https://pnpm.io/) (version 7 or higher)
- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

## Setup Instructions

1. **Clone the repository**:
    ```sh
    git clone https://github.com/julklingel/scanlytics
    cd scanlytics
    ```

2. **Install dependencies**:
    ```sh
    pnpm install
    ```

3. **Set up the Rust environment**:
    Ensure you have Rust installed. You can install it using [rustup](https://rustup.rs/):
    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

4. **Build the Sveltekit Project**:
    ```sh
    pnpm build
    ```

5. **Run the Tauri application in dev mode**:
    ```sh
    pnpm tauri dev
    ```

6. **Build the project**:
    ```sh
    pnpm tauri build
    ```

## Scripts

- `pnpm tauri dev`: Starts the Tauri development server.
- `pnpm tauri build`: Builds the Tauri application.

- `cargo run`: Runs the Rust backend.
- `cargo build`: Builds the Rust backend.
- `cargo check`: Runs type checking for the Rust backend.
- `cargo clean`: Cleans the Rust backend build artifacts.
- `cargo fmt`: Formats the Rust backend code.
- `cargo clippy`: Runs linter for the Rust backend.


## Configuration

### Tauri Configuration

The Tauri configuration is located in `src-tauri/tauri.conf.json`. This file contains settings for the Tauri application, including build commands, window settings, and security options.

### Rust Dependencies

Rust dependencies are managed in `src-tauri/Cargo.toml`. This file includes dependencies for Tauri, Serde, and other Rust libraries used in the project.

### JavaScript/TypeScript Dependencies

JavaScript and TypeScript dependencies are managed in `package.json` and `pnpm-lock.yaml`. These files include dependencies for SvelteKit, Vite, Tauri, and other libraries used in the project.

## License

This project is licensed under the MIT License.


