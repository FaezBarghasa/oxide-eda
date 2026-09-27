---
okf_version: "0.2"
type: Function
title: synthesize_manifest
resource: crates/oxide-library/src/adapters/local_git/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/helpers/synthesize_manifest
language: rust
---

# synthesize_manifest

## Signature

```rust
pub(super) fn synthesize_manifest(snx: &SnxlibManifest) -> Manifest
```

## Visibility

- `pub(super)`

## Source
Lines 123–139 in `crates/oxide-library/src/adapters/local_git/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-library/src/adapters/local_git/helpers.md) |
| called_by | [init](/crates/oxide-library/src/adapters/local_git/mod/init.md) |
| called_by | [open](/crates/oxide-library/src/adapters/local_git/mod/open.md) |
| called_by | [recover_init](/crates/oxide-library/src/adapters/local_git/mod/recover_init.md) |
