# manta-signer

`manta-signer` is manta's native client that **turbo charges** zero-knowledge-proof generation.

<p align="center">
    <img width="655" src="./mac-installation.png">
</p>

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
