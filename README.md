# VEX V5 Template for the Rust Programming Language

## Table of Contents

- [VEX V5 Template for the Rust Programming Language](#vex-v5-template-for-the-rust-programming-language)
  - [Table of Contents](#table-of-contents)
  - [Getting Started (macOS)](#getting-started-macos)
  - [Getting Started (Windows)](#getting-started-windows)
  - [Getting Started (Fedora Linux)](#getting-started-fedora-linux)
  - [Getting Started (Debian/Ubuntu linux)](#getting-started-debianubuntu-linux)
  - [Development](#development)
    - [Compiling and uploading to a VEX V5 robot](#compiling-and-uploading-to-a-vex-v5-robot)
    - [Debugging in the pros-rs simulator](#debugging-in-the-pros-rs-simulator)
    - [Using smart editing features](#using-smart-editing-features)
  - [Troubleshooting](#troubleshooting)


## Getting Started (macOS)

Run the following terminal commands to set up your Mac for development.

Install Homebrew, a package manager for macOS which is needed to configure your computer with the necessary development tools:

```console
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Under the header "Next Steps", Homebrew may prompt you to run commands to complete the installation.

Install dependencies and The Rust Programming Language:

```console
brew install rustup osx-cross/arm/arm-gcc-bin purduesigbots/pros/pros-cli

rustup-init -y --default-toolchain nightly
```

Close and reopen the terminal, and install pros-rs:

```console
rustup component add rust-src
cargo install cargo-pros
```

## Getting Started (Windows)

Install Rust by following the instructions on <https://rustup.rs/>. Configure your installation when prompted, installing a **nightly** toolchain and keeping other default values.

Download and run the [ARM EABI Toolchain](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads) installer, choosing the first `.exe` file under the header `arm-none-eabi`.

Install Python from <https://python.org/>.

Run the following commands in Powershell to install pros-rs:

```
python3 -m pip install --user pros-cli
rustup component add rust-src
cargo install cargo-pros
```

## Getting Started (Fedora Linux)

Run the following terminal commands to install dependencies and set up your PC for development.

```console
sudo dnf install rustup python3-pip arm-none-eabi-gcc-cs gcc

rustup-init -y --default-toolchain nightly
pip install --user pros-cli
```

Close and reopen the terminal, and install pros-rs:

```console
rustup component add rust-src
cargo install cargo-pros
```

## Getting Started (Debian/Ubuntu linux)

If you don't have rustup:
```console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup override set nightly
```

If you do have rustup:
```console
rustup update
rustup toolchain install nightly
rustup override set nightly
```

Install pip and the Pros CLI:
```console
sudo apt install python3-pip
pip install --user pros-cli
```

Close and reopen the terminal, and install pros-rs:

```console
rustup component add rust-src --toolchain nightly
cargo install cargo-pros
```

## Development

### Compiling and uploading to a VEX V5 robot

Use the Cargo PROS terminal utility to compile this pros-rs project.

```console
cargo pros build
```

The separate `pros` command is used to upload. Plug in your VEX robot brain via USB and run the following command to upload to program slot 1:

```console
pros upload --target v5 --slot 1 ./target/armv7a-vexos-eabi/debug/pros-template-rust.bin
```

### Debugging in the pros-rs simulator

If you have PROS Simulator installed, you can use it to run this project without real VEX hardware for debugging and development purposes. Start by adding the WebAssembly Rust target:

```console
rustup target add wasm32-unknown-unknown
```

Build the project for the simulator by running:

```console
cargo pros build -s
```

Then open this project in PROS Simulator to run and debug the robot code.

### Using smart editing features

After building the project for the first time, developers using Visual Studio Code and rust-analyzer have access to smart editing features like Intellisense and code analysis. By default, rust-analyzer will check the project's code for errors when it is saved.

## Troubleshooting

- If you experience issues with Intellisense, make sure you've [built the project](#compiling-and-uploading-to-a-vex-v5-robot) at least once!
