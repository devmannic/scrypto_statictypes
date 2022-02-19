# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.3.1] - 2022-02-19
### Added
- Common impl for From<$w> for $t to automatically unwrap making API usage cleaner
- `Account::deposit_of::<RESOURCE>` and `Account::withdraw_of::<RESOURCE>`
- Comparison between ResourceDef and ResourceOf with == and !=
### Fixed
- warnings on resource names (any case is now allowed)
- missing trait in prelude hiding BucketRefOf::unchecked_into

## [0.3.0] - 2021-12-24
### Added
- Implemented ResourceOf and BucketRefOf
- Added more tests
### Changed
- Bucket and Vault container types - have methods which require ResourceOf and BucketRefOf
- Refactored with macros for better code reuse while retaining good error messages
- Runtime checks ensure resource name to address mapping is 1:1 to catch certain errors

## [0.2.0] - 2021-12-16
### Changed
- Compatibility updates for Alexandria Scrypto v0.2.0

## [0.1.1] - 2021-12-16
### Changed
- Pin versions to pre-Alexandria Scrypto v0.1.1

## [0.1.0] - 2021-12-01
### Added
- Initial Version
