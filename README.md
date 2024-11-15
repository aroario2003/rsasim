# Rsasim

An rsa encryption algorithm simulator

## Installing rust

Install rustup:

Linux:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Windows:

Refer to [this page](https://rust-lang.github.io/rustup/installation/other.html])

## Install Stable Toolchain

```shell
rustup install --profile minimal
```

## Building with cargo

```shell
cargo build --release
```

## Running

From the root of the project (if built with `cargo build`):

```shell
./target/release/rsasim <options>...
```
or

```shell
cargo run --release -- <options>...
```

