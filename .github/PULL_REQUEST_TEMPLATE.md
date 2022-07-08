<!-- < < < < < < < < < < < < < < < < < < < < < < < < < < < < < < < < < ☺
v                               ✰  Thanks for creating a PR! ✰
v    Before hitting that submit button please review the checkboxes.
v    If a checkbox is n/a - please still include it but + a little note why
☺ > > > > > > > > > > > > > > > > > > > > > > > > > > > > > > > > >  -->

## Description

<!-- Add a description of the changes that this PR introduces and the files that
are the most critical to review.
-->

closes: #XXXX

---

Before we can merge this PR, please make sure that all the following items have been
checked off. If any of the checklist items are not applicable, please leave them but
write a little note why.

- [ ] Linked to Github issue with discussion and accepted design OR have an explanation in the PR that describes this work.
- [ ] Test following procedure in docs/testing-workflow.md
- [ ] Updated relevant documentation in the code.
- [ ] Re-reviewed `Files changed` in the Github PR explorer.

## If release PR
- [ ] Update the version numbers properly:
   * Cargo.toml
   * ui/package.json
   * ui/public/about.html
   * ui/src-tauri/Cargo.toml
   * ui/src-tauri/tauri.conf.json
   * www/src/App.tsx

## After release
- [ ] Verify that [the signer download site](https://signer.manta.network/) works correctly
- [ ] Verify that all jobs to build signer for all operating systems were successful
- [ ] Verify that the previous signer release is prompted for an update upon sign in
