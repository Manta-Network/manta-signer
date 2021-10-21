# manta-signer

_Manta Signer Server_

NB: Currently, the root seed is saved in the current directory instead of a standard location.
NB: Currently, the proving keys are not automatically generated, but are read directly from the current directory.

## Development

To build and package the signer UI, you'll need to install the [`tauri`](https://github.com/tauri-apps/tauri) CLI. The simplest way to install `tauri-cli`, if you use `Rust`, is to install it using `cargo`:

```sh
cargo install tauri-cli --version ^1.0.0-beta
```

See the `tauri` docs for `npm` or `yarn` installation of the `tauri-cli`.

### Building

To build the project go to the `src-tauri` directory and run any `cargo` commands like `cargo build`, `cargo clippy`, or `cargo doc`. To run the executable, use the `cargo tauri` command (installed above using `cargo`):

```sh
cargo tauri dev
```

### Packaging

To package the binary for distribution, run `cargo tauri build`.
