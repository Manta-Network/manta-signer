# Manta client end to end test

Test randomly simulating

### How to run

0. Setup the simulation file `cd simulation && cargo run -- -o out.json 10`
1. Run a local Manta node
2. Run four instances of [`manta-signer`](https://github.com/Manta-Network/manta-signer) test server on localhost ports 29987-29990. It might be convenient to add aliases to your bash profile:

```bash
alias alice_signer="cd /path/to/manta-signer/examples && cargo run --example test_server --release -- http://127.0.0.1:29987"
alias bob_signer="cd /path/to/manta-signer/examples && cargo run --example test_server --release -- http://127.0.0.1:29988"
alias charlie_signer="cd /path/to/manta-signer/examples && cargo run --example test_server --release -- http://127.0.0.1:29989"
alias dave_signer="cd /path/to/manta-signer/examples && cargo run --example test_server --release -- http://127.0.0.1:29990"
```

3. From the root of `manta-js`, run `yarn install && yarn build && yarn test`
