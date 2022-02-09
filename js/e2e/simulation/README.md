# manta-simulation

This is a temporary simulation suite for possible transactions on the Manta Network. This should only be used for internal testing at the moment. Eventually, this should be part of a larger, more comprehensive, end-to-end testing solution.

## Running the Simulation

To run the simulation, use `cargo run -- <OPTIONS>`. For the list of options use `cargo run -- --help`.

## Configuration

See the `Config` struct in `src/main.rs` for documetation on configuration options for the simulator and see `default-config.json` for the default configuration. The default configuration is automatically used by `cargo run` if the `--config` option is not passed.

For more documentation run `cargo doc --open`.
