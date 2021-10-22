# Manta Signer Examples

## Test Server

To run the `test_server` example, use the following:

```sh
cargo run --example test_server -- <URL>...
```

where `<URL>` are the locations that the test server should listen for requests.

NB: The `test_server` example is not part of the integration `tests` directory because it would then run as part of the normal test suite.
