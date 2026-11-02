# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

Changelog is partly derived from Clap changelog:
TODO: add URL

Changelog notes for each version may be split into:

 - Breaking changes: user-visible changes which may require updates in downstream components.
 - Compatibility: which compiler and dependency versions are supported.
 - Features: new functionality, or backwards-compatible changes in existing functionality.
 - Fixes: bugfixes, security fixes, removing sharp edges.
 - Performance: performance-oriented improvements: memory-usage, cpu usage, etc.
 - Releases: ready-to-run compiled binaries which may be distributed via Github Releases
    or other distribution systems.


## [0.14.0-rust-2018-edition] - 2026-11-02
### Breaking Changes
 - Upgrade to Rust 2018 edition
   Required Rust code updates:
    1. try!(...) replaced by (...)?
    2. error::Error now prefixed with crate::error::Error

### Compatibility: Compiled using
`rust-toolchain.toml`

```
[toolchain]
channel = "nightly-2024-12-15"
```

OR

```
[toolchain]
channel = "nightly-2025-05-09"
```

## [0.13.0] - 2020-10-02
Please see `git log -p` for changes.

### Compatibility: Compiled using
`rust-toolchain.toml`

```
[toolchain]
channel = "nightly-2024-12-15"
```

OR

```
[toolchain]
channel = "nightly-2025-05-09"
```