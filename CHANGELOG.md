# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed
- [\#338](https://github.com/Manta-Network/manta-signer/pull/338) Change start button to continue in View zkAddress.
### Deprecated

### Removed

### Fixed
- [\#329](https://github.com/Manta-Network/manta-signer/pull/329) Fix bad sync after reimport or password reset

### Security

## [1.1.1] 2023-02-13
### Added

### Changed

### Deprecated

### Removed

### Fixed
- [\#292](https://github.com/Manta-Network/manta-signer/pull/292) Fix linux signer not finding proving keys on system
- [\#289](https://github.com/Manta-Network/manta-signer/pull/289) Fix authorization state preventing consecutive transactions

### Security

## [1.0.1] 2023-01-24
### Added
- [\#261](https://github.com/Manta-Network/manta-signer/pull/261) Bundle Proving Keys with manta-signer (no download anymore)

### Changed
- [\#252](https://github.com/Manta-Network/manta-signer/pull/252) Change app name to manta-signer and coloured icons
### Deprecated

### Removed

### Fixed
- [\#250](https://github.com/Manta-Network/manta-signer/pull/250) Fix View recovery phrase window not opening and ZkAddress window closure closing the whole app sometimes
- [\#236](https://github.com/Manta-Network/manta-signer/pull/236) Fix missing file reload

### Security

## [1.0.0] 2022-11-23

### Added
- [\#228](https://github.com/Manta-Network/manta-signer/pull/228) Adding support for new V3 protocol(AES) and bug fixes (UI).
- [\#215](https://github.com/Manta-Network/manta-signer/pull/215) Added signer redesign UI, account deletion and recovery.

### Fixed
- [\#199](https://github.com/Manta-Network/manta-signer/pull/199) Fix unable to login to signer if close during state save
- [\#204](https://github.com/Manta-Network/manta-signer/pull/204) Check for storage file before deleting it

## [0.8.0] 2022-10-05

### Added
- [\#180](https://github.com/Manta-Network/manta-signer/pull/180) Add support for mutliple CORS allowed origins

## [0.7.4] - 2022-09-27
### Added
- [\#154](https://github.com/Manta-Network/manta-signer/pull/154) Add storage abstractions and add server storage hook

### Fixed
- [\#166](https://github.com/Manta-Network/manta-signer/pull/166) Stop close button[X] on Authorization prompt(Private to Anything) from closing manta-signer
- [\#164](https://github.com/Manta-Network/manta-signer/pull/164) Adding some communication between UI and backend to ensure sync at connection start

## [0.7.3] - 2022-07-11T17:25:22Z
### Fixed
- [\#131](https://github.com/Manta-Network/manta-signer/pull/131) Fix window hang

## [0.7.2] - 2022-07-06T18:10:32Z
### Changed
- [\#119](https://github.com/Manta-Network/manta-signer/pull/119) Update parameter path from SDK to `manta-parameters`

### Fixed
- [\#135](https://github.com/Manta-Network/manta-signer/pull/135) Fix download links

## [0.7.1] - 2022-06-17T22:25:30Z
### Changed
- [\#116](https://github.com/Manta-Network/manta-signer/pull/116) Upgrade tauri and add some messages

## [0.7.0] - 2022-06-15T20:55:48Z
### Added
- [\#74](https://github.com/Manta-Network/manta-signer/pull/74) Add `dependabot` configuration
- [\#76](https://github.com/Manta-Network/manta-signer/pull/76) Create auto-release pipeline
- [\#89](https://github.com/Manta-Network/manta-signer/pull/89) Move landing page from `manta-signer-install`
- [\#97](https://github.com/Manta-Network/manta-signer/pull/97) Add password hint
- [\#85](https://github.com/Manta-Network/manta-signer/pull/85) Add Rust CI

### Changed
- [\#103](https://github.com/Manta-Network/manta-signer/pull/103) Change release workflow

### Fixed
- [\#84](https://github.com/Manta-Network/manta-signer/pull/84) Use the correct `tag_name` argument in CI
- [\#87](https://github.com/Manta-Network/manta-signer/pull/87) Remove macOS 10.15 from CI
- [\#96](https://github.com/Manta-Network/manta-signer/pull/96) Upgrade to new synchronization protocol
- [\#107](https://github.com/Manta-Network/manta-signer/pull/107) Highlight recovery phrase selection
- [\#109](https://github.com/Manta-Network/manta-signer/pull/109) Remove whitespace at the bottom of the signer install page

## [0.6.0] - 2022-05-19T20:47:37Z
### Added
- [\#45](https://github.com/Manta-Network/manta-signer/pull/45) Add ubuntu 18 support
- [\#52](https://github.com/Manta-Network/manta-signer/pull/52) Add JS libraries
- [\#54](https://github.com/Manta-Network/manta-signer/pull/54) Add auto-updater

### Changed
- [\#43](https://github.com/Manta-Network/manta-signer/pull/43) Get Proving Keys from Git-LFS
- [\#53](https://github.com/Manta-Network/manta-signer/pull/53) Move to new SDK which requires legacy file locations
- [\#57](https://github.com/Manta-Network/manta-signer/pull/57) Integration with `manta-rs`

### Fixed
- [\#67](https://github.com/Manta-Network/manta-signer/pull/67) Update `README.md`

## [0.5.1] - 2022-01-13T21:31:09Z
### Added
- [\#40](https://github.com/Manta-Network/manta-signer/pull/40) Add tagging workflow

### Changed
- [\#42](https://github.com/Manta-Network/manta-signer/pull/42) Update copywrite text

## [0.5.0] - 2021-12-27T17:56:52Z
### Added
- [\#32](https://github.com/Manta-Network/manta-signer/pull/32) Support Windows Release
- [\#31](https://github.com/Manta-Network/manta-signer/pull/31) Close app on quit window
- [\#30](https://github.com/Manta-Network/manta-signer/pull/30) Add unsafe CORS disable feature for development builds

### Fixed
- [\#36](https://github.com/Manta-Network/manta-signer/pull/36) Improve wallet recovery performance

## [0.4.1] - 2021-12-08T21:52:47Z
### Added
- [\#4](https://github.com/Manta-Network/manta-signer/pull/4) Integrate with frontend
- [\#14](https://github.com/Manta-Network/manta-signer/pull/14) New Signer UI
- [\#17](https://github.com/Manta-Network/manta-signer/pull/17) Add about page with version number
- [\#21](https://github.com/Manta-Network/manta-signer/pull/21) Add `README.md`
- [\#23](https://github.com/Manta-Network/manta-signer/pull/23) Add Testnet transaction summary

### Changed
- [\#10](https://github.com/Manta-Network/manta-signer/pull/10) Upgrade to Signer v2
- [\#13](https://github.com/Manta-Network/manta-signer/pull/13) Rewrite Signer in Rust

### Fixed
- [\#25](https://github.com/Manta-Network/manta-signer/pull/25) Fix issues with password retry and UI
- [\#3](https://github.com/Manta-Network/manta-signer/pull/3) Translate to English

[Unreleased]: https://github.com/Manta-Network/manta-signer/compare/v0.7.3...HEAD
[0.7.3]: https://github.com/Manta-Network/manta-signer/releases/tag/v0.7.3
[0.7.2]: https://github.com/Manta-Network/manta-signer/releases/tag/v0.7.2
[0.7.1]: https://github.com/Manta-Network/manta-signer/releases/tag/v0.7.1
[0.7.0]: https://github.com/Manta-Network/manta-signer/releases/tag/v0.7.0
[0.6.0]: https://github.com/Manta-Network/manta-signer/releases/tag/v0.6.0
[0.5.1]: https://github.com/Manta-Network/manta-signer/releases/tag/v0.5.1
[0.5.0]: https://github.com/Manta-Network/manta-signer/releases/tag/v0.5.0
[0.4.1]: https://github.com/Manta-Network/manta-signer/releases/tag/v0.4.1
