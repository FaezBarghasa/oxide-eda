---
okf_version: "0.2"
type: Function
title: from_snxlib
description: "Construct from a [`SnxlibManifest`] — the v0.9 manifest shape."
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/from_snxlib_1
language: rust
---

# from_snxlib

Construct from a [`SnxlibManifest`] — the v0.9 manifest shape.

## Signature

```rust
pub fn from_snxlib(manifest: SnxlibManifest) -> Result<Self, LibraryError>
```

## Visibility

- `pub`

## Docstring

Construct from a [`SnxlibManifest`] — the v0.9 manifest shape.

The DB adapter has no `.snxlib` file on disk, but it still
carries the same library metadata in [`SnxlibManifest::mode`]
(must be [`LibraryMode::Database`]). This constructor mirrors
[`crate::adapters::local_git::LocalGitAdapter::init`]'s API
surface so callers reaching for one or the other can use the
same manifest type.

Synthesises the legacy [`Manifest`] internally for the
[`LibraryAdapter::manifest`] callers — Stage 5+ retires those
and lets us drop the synthesis.

## Source
Lines 102–104 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
| calls | [synthesize_legacy_manifest](/crates/oxide-library/src/adapters/database/synthesize_legacy_manifest.md) |
