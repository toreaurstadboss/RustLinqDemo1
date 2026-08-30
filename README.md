# rustlinqdemo1

A small Rust demo project that shows a custom `first_or_default()` helper and is ready to run and debug from Visual Studio Code with CodeLLDB.

## What you need

Install these tools first:

- Rust compiler and Cargo: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- Visual Studio Code: [https://code.visualstudio.com/Download](https://code.visualstudio.com/Download)
- Rust Analyzer extension: [https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- CodeLLDB extension: [https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

If you install Rust with rustup, Cargo is included automatically.

## Run the project

From the project root:

```powershell
cargo run
```

## Test the project

From the project root:

```powershell
cargo test
```

## Debug in VS Code

1. Open this folder in VS Code.
2. Make sure the Rust Analyzer and CodeLLDB extensions are installed.
3. Open the Run and Debug view.
4. Select `Debug rustlinqdemo1 (CodeLLDB)`.
5. Press `F5`.

The debug configuration is in [.vscode/launch.json](.vscode/launch.json).

## Project files

- `src/main.rs`: program entry point and demo code.
- `Cargo.toml`: crate metadata and dependencies.
- `.vscode/launch.json`: VS Code debug configuration.
