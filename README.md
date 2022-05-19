# manta-signer

`manta-signer` is manta's native client that **turbo charges** zero-knowledge-proof generation.

<img width="655" src="https://user-images.githubusercontent.com/720571/142786609-ce7455e1-dbe7-4a6d-8a78-4aa22984a3d7.png">

**Disclaimer: `manta-signer` is experimental software, use it at your own risk.**

## Project Organization

- `src`: `manta-signer` ZKP generation
- `ui`: `manta-signer` Desktop UI
- `js`: external Javascript libraries that interact with `manta-signer`

## Restarting Your Account

To remove your private account data completely and set up a new account, you should remove these files:

- macOS: `~/Library/Application Support/manta-signer/`
- Linux: `~/.config/manta-signer/`
- Windows: `~/AppData/Roaming/manta-signer/`
