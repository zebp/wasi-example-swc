# wasi-example-swc

A sample wasi worker using the [SWC](https://swc.rs) JavaScript compiler.

## Installing Prerequisites

While compiling Rust to wasi is usually fairly easy, in this case we have to put in some extra work to get the binary as small as we can.

### Installing Rust

This being a Rust example, you'll obviously need the rust toolchain and the wasi target installed.

```bash
# Installs the rust toolchain manager from the official site
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Installs everything we'll need to compile to the wasi target.
$ rustup target add wasm32-wasi
```

### Installing Binaryen

[Binaryen](https://github.com/WebAssembly/binaryen) is a full blown WebAssemly compiler and toolchain library that ships with some useful tools. In this case we only care about the `wasm-opt` tool, which we'll use to run the generated wasm module though the Binaryen compiler to aggressively optimize for size. This is super easy for UNIX-like operating system users, but windows will have to look at the [Binaryen](https://github.com/WebAssembly/binaryen) repo.

```bash
# Debian/Ubuntu/Kali
apt-get install binaryen
# Arch Linux
pacman -S binaryen
# OS X
brew install binaryen
```

## Building

Although installing all the prerequisites can be cumbersome, getting the final binary is actually really easy!

```bash
# Build the initial wasm module from our Rust source
$ cargo build --release --target wasm32-wasi
# Strip out everything that isn't necessary and then optimize for size
$ wasm-opt --strip-debug --strip-producers --strip-target-features -Oz ./target/wasm32-wasi/release/swc-wasi.wasm --output ./target/wasm32-wasi/release/swc-wasi.wasm
```

And that's it! You're ready to play with your worker!

## Running it

Thankfully [wrangler2](https://github.com/cloudflare/wrangler2/) makes interacting with our wasm super easy, all we need to do to run swc is:

`npx wrangler@wasm dev ./target/wasm32-wasi/release/swc-wasi.wasm`

### Note

Currently WASI support inside of [wrangler2](https://github.com/cloudflare/wrangler2/) has not landed in the main branch, but is published to NPM under an experimental `wasm` tag.

