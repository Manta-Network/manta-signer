# manta-signer

`manta-signer` is manta's native client that **turbo charges** zero-knowledge-proof generation.

<img width="655" src="https://user-images.githubusercontent.com/720571/142786609-ce7455e1-dbe7-4a6d-8a78-4aa22984a3d7.png">

Disclaimer: `manta-signer` is experimental software, use it at your own risk.

## Project Organization
- `src`: `manta-signer` zkp generation
- `ui`: `manta-signer` desktop UI
- `js`: external Javascript libraries that interact with `manta-signer`

<br/>
To remove your private account data completely and set up a new account, you should remove these files.

```
Mac: ~/Library/Application Support/manta-signer/root_seed.aes
Linux: ~/.config/manta-signer/root_seed.aes
Windows: ~/AppData/Roaming/manta-signer/root_seed.aes
```
