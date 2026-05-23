# tooler

A modular CLI toolkit built with Rust. Designed as a boilerplate — add your own commands on top of it.

## Installation

### Option 1 — curl (no Rust required)

Downloads a prebuilt binary for your OS and architecture:

```sh
curl -fsSL https://raw.githubusercontent.com/venturaproject/tooler/master/install.sh | sh
```

Supports: macOS (Intel + Apple Silicon), Linux (x86_64 + arm64).

To install a specific version:

```sh
TOOLER_VERSION=v1.0.0 curl -fsSL https://raw.githubusercontent.com/venturaproject/tooler/master/install.sh | sh
```

### Option 2 — cargo (requires Rust)

If you already have Rust installed:

```sh
cargo install --git https://github.com/venturaproject/tooler
```

Or from a local clone:

```sh
git clone https://github.com/venturaproject/tooler
cd tooler
cargo install --path .
```

### Install Rust (only needed for Option 2)

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

## Requirements

| Method | Requirement |
|---|---|
| curl install | nothing — binary is self-contained |
| cargo install | Rust 1.70+ and Cargo |
| Build from source | Rust 1.70+ and Cargo |

Supported platforms: macOS, Linux, Windows (Windows via cargo only).

## Verify installation

```sh
tooler --version
```

If the command is not found, add the install directory to your PATH:

```sh
# if installed to ~/.local/bin
export PATH="$HOME/.local/bin:$PATH"

# if installed via cargo
export PATH="$HOME/.cargo/bin:$PATH"
```

Then add that line to your `~/.zshrc` or `~/.bashrc`.

## Uninstall

```sh
# if installed via curl
sudo rm /usr/local/bin/tooler
# or
rm ~/.local/bin/tooler

# if installed via cargo
cargo uninstall tooler
```

---

## Usage

```sh
tooler <command> [options]
tooler --help
tooler <command> --help
```

### `tooler info`

Show system information (working directory and environment variables).

```sh
tooler info            # show everything
tooler info --dir      # working directory only
tooler info --env      # environment variables only
```

### `tooler echo`

Echo text with optional color and formatting.

```sh
tooler echo hello world
tooler echo hello world --color green
tooler echo hello world --upper --color cyan
tooler echo hello --repeat 5
```

Available colors: `red`, `green`, `blue`, `yellow`, `cyan`, `magenta`

### `tooler json`

Pretty-print and query JSON from a file or stdin.

```sh
tooler json file.json                      # pretty-print a file
tooler json file.json --key user.name      # extract a nested field
tooler json file.json --compact            # compact output
cat file.json | tooler json                # read from stdin
cat file.json | tooler json --key items    # stdin + key extraction
```

---

## Extending tooler

To add a new command, follow these steps:

**1.** Create `src/commands/my_command.rs`:

```rust
use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct MyCommandArgs {
    pub input: String,
}

pub fn run(args: MyCommandArgs) -> Result<()> {
    println!("input: {}", args.input);
    Ok(())
}
```

**2.** Register in `src/commands/mod.rs`:

```rust
pub mod my_command;
```

**3.** Add to `Commands` in `src/cli.rs`:

```rust
/// Description shown in --help
MyCommand(my_command::MyCommandArgs),
```

**4.** Handle in `src/main.rs`:

```rust
Commands::MyCommand(args) => commands::my_command::run(args),
```

**5.** Reinstall:

```sh
cargo install --path .
```

## Releasing a new version

Tag a commit and GitHub Actions will build binaries for all platforms automatically:

```sh
git tag v1.0.0
git push origin v1.0.0
```

The release workflow builds for:
- `linux/x86_64`
- `linux/aarch64`
- `macos/x86_64` (Intel)
- `macos/aarch64` (Apple Silicon)
- `windows/x86_64`

## Dependencies

| Crate | Purpose |
|---|---|
| `clap` | Argument parsing and subcommand structure |
| `anyhow` | Ergonomic error handling |
| `colored` | Terminal color output |
| `serde` + `serde_json` | JSON serialization |
| `thiserror` | Custom error types |
