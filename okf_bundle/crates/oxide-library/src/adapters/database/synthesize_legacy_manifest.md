---
okf_version: "0.2"
type: Function
title: synthesize_legacy_manifest
description: "Build a legacy [`Manifest`] from the v0.9 [`SnxlibManifest`] header."
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/synthesize_legacy_manifest
language: rust
---

# synthesize_legacy_manifest

Build a legacy [`Manifest`] from the v0.9 [`SnxlibManifest`] header.

## Signature

```rust
fn synthesize_legacy_manifest(snx: SnxlibManifest) -> Manifest
```

## Docstring

Build a legacy [`Manifest`] from the v0.9 [`SnxlibManifest`] header.

The DB adapter's `manifest()` trait method continues to return a
`&Manifest` so existing callers (`new_component`, `dispatch/library`,
`state.rs`) keep working. Stage 5+ will retire this synthesis once
those callers move onto the new accessors.

## Source
Lines 518–532 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
| called_by | [from_snxlib](/crates/oxide-library/src/adapters/database/from_snxlib.md) |
