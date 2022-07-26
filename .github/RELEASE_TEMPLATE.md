## Release vX.Y.Z

**Each reviewer needs to check that these conditions are met before approving the PR.**

- [ ] Checked that the release is on the correct branch name of the form `release-vX.Y.Z` 
- [ ] Added the `L-skip` label and the relevant `release` label to this PR
- [ ] Updated the [`CHANGELOG.md`](https://github.com/manta-network/manta-signer/blob/main/CHANGELOG.md)
- [ ] Updated the version numbers in the following files:
    - [ ] [`Cargo.toml`](https://github.com/manta-network/manta-signer/blob/main/Cargo.toml)
    - [ ] [`ui/package.json`](https://github.com/manta-network/manta-signer/blob/main/ui/package.json)
    - [ ] [`ui/public/about.html`](https://github.com/manta-network/manta-signer/blob/main/ui/public/about.html)
    - [ ] [`ui/src-tauri/Cargo.toml`](https://github.com/manta-network/manta-signer/blob/main/ui/src-tauri/Cargo.toml)
    - [ ] [`ui/src-tauri/tauri.conf.json`](https://github.com/manta-network/manta-signer/blob/main/ui/src-tauri/tauri.conf.json)
    - [ ] [`www/src/App.tsx`](https://github.com/manta-network/manta-signer/blob/main/www/src/App.tsx)
- [ ] Run the testing procedure defined in [`docs/testing-workflow.md`](https://github.com/manta-network/manta-signer/blob/main/docs/testing-workflow.md)

### After the Release

- [ ] Verify that [the signer download site](https://signer.manta.network) works correctly
- [ ] Verify that all jobs to build signer for all operating systems were successful
- [ ] Verify that the previous signer release is prompted for an update upon sign in
