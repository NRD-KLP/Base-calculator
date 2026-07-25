# Base Calculator

A console calculator written in Rust. It performs basic arithmetic operations, handles exponentiation and percentages, and features safety protection against division by zero.

### Features
* Asks for two numbers.
* Supports operations: `+`, `-`, `*`, `/`, exponentiation (`^`), and percentages (`%`).
* Contains protection against division by zero.

---

## Prerequisites (Requirements)

Before running this project, you need to install the Rust toolchain (which includes Cargo) and a code editor.

### 1. Install Rust & Cargo
Go to the official website [rustup.rs](https://rustup.rs) and follow the instructions for your operating system:
* **Windows**: Download and run `rustup-init.exe`.
* **macOS / Linux**: Run the following command in your terminal:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://rustup.rs | sh
  ```

To verify the installation, restart your terminal and run:
```bash
cargo --version
```

### 2. Recommended Extensions (for VS Code)
If you are using **Visual Studio Code**, it is highly recommended to install these extensions from the Marketplace for a better coding experience:
* **rust-analyzer** — The official extension for autocompletion, code navigation, and real-time error checking.
* **Even Better TOML** — Provides syntax highlighting and validation for the `Cargo.toml` file.
* **Code Runner** (Optional) — Allows you to run the code quickly using a play button.

---

## How to Run

1. Open your terminal or command prompt.
2. Navigate to the project folder:
   ```bash
   cd base_calculator
   ```
3. Run the project using Cargo:
   ```bash
   cargo run
   ```
