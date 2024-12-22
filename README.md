# Scanlytics

Scanlytics is an advanced medical imaging application that enhances radiologists' report writing experience by automatically analyzing medical images and generating structured reports. It combines machine learning-based image analysis with medical knowledge management.

## Features

- **Authentication System**: Secure user authentication and session management
- **User Management**: Role-based access control and user administration
- **Patient Management**: Comprehensive patient record handling
- **Medical Notes**: Structured medical note creation and management
- **Report Generation**: Automated medical report generation
- **Image Analysis**: ML-powered medical image processing and classification

## Architecture

### Frontend

- SvelteKit with TypeScript
- Tauri integration for desktop functionality
- Responsive UI for medical workflows

### Backend Components

#### Authentication Module

- User authentication and token management
- Session handling
- Security middleware

#### Patient Management

- Patient record CRUD operations
- Medical history tracking
- Relationship management

#### Medical Notes

- Structured note creation
- Patient-note relationships
- Note categorization

#### Report Generation

- Automated report structure
- Image-text correlation

## Project Structure

```plaintext
src-tauri/
├── src/
│   ├── auth/           # Authentication module
│   ├── users/          # User management
│   ├── patients/       # Patient management
│   ├── notes/          # Medical notes
│   ├── reports/        # Report generation
│   └── image_analysis/ # Image processing and ML
│       ├── image_processing/
│       └── ml_models/
└── db/
    ├── src/            # Database workspace
    └── Cargo.toml      # Rust dependencies
```


## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/)
  - [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


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

4. **Build the SvelteKit project**:
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
- `cargo doc --open`: Generates and opens the Rust backend documentation.
- `pnpm tauri dev`: Starts the Tauri development server.
- `pnpm tauri build`: Builds the Tauri application.
- `cargo build`: Builds the Rust backend.
- `cargo check`: Runs type checking for the Rust backend.
- `cargo clean`: Cleans the Rust backend build artifacts.
- `cargo fmt`: Formats the Rust backend code.
- `cargo clippy`: Runs linter for the Rust backend.

## Configuration

### Tauri Configuration

The Tauri configuration is located in [tauri.conf.json](http://_vscodecontentref_/2). This file contains settings for the Tauri application, including build commands, window settings, and security options.

### Database Configuration

Scanlytics supports switching between file-based persistent storage and in-memory storage. This can be configured by passing a boolean value to the `init_db` function.

- **File-based Persistent Storage**: Pass `false` as the second argument to `init_db`.
- **In-memory Storage**: Pass `true` as the second argument to `init_db`.

The in-memory storage option is useful for testing and development purposes, while the file-based storage option is recommended for production use.


### Rust Dependencies

Rust dependencies are managed in [Cargo.toml](http://_vscodecontentref_/3). This file includes dependencies for Tauri, Serde, and other Rust libraries used in the project.

### JavaScript/TypeScript Dependencies

JavaScript and TypeScript dependencies are managed in [package.json](http://_vscodecontentref_/4) and [pnpm-lock.yaml](http://_vscodecontentref_/5). These files include dependencies for SvelteKit, Vite, Tauri, and other libraries used in the project.

## Running Tests

To run the tests for the Rust backend, use the following command:
```sh
cargo test --features test-utils
```
To run the tests for the SvelteKit frontend, use the following command:
```sh
pnpm test
```

