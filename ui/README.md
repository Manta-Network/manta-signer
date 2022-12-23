# Manta Signer UI

NB: Currently, the proving keys are not automatically generated.

## Development

To build and package the signer UI, you'll need to install the [`tauri`](https://github.com/tauri-apps/tauri) CLI. The simplest way to install `tauri-cli`, if you use `Rust`, is to install it using `cargo`:

```sh
cargo install tauri-cli --version ^1.0.0-beta
```

See the `tauri` docs for `npm` or `yarn` installation of the `tauri-cli`.

### Building

To build the project go to the `src-tauri` directory and run any `cargo` commands like `cargo build`, `cargo clippy`, or `cargo doc`. To run the executable, first run `yarn start`, then run `cargo tauri dev --features=disable-restart` (installed above using `cargo`) in a separate terminal window:

Note that ZKP generation will be very slow for dev builds

### Packaging

To package the binary for distribution, run `yarn bundle`.
