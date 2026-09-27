---
okf_version: "0.2"
type: Function
title: scan_symbol_files
description: ── Symbol container helpers (v0.9 phase 2 multi-symbol files) ────────
resource: crates/oxide-library/src/adapters/local_git/primitives.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/local_git/primitives/scan_symbol_files_1
language: rust
---

# scan_symbol_files

── Symbol container helpers (v0.9 phase 2 multi-symbol files) ────────

## Signature

```rust
pub(super) fn scan_symbol_files(&self) -> Result<Vec<(PathBuf, SymbolFile)>, LibraryError>
```

## Visibility

- `pub(super)`

## Docstring

── Symbol container helpers (v0.9 phase 2 multi-symbol files) ────────

## Source
Lines 304–338 in `crates/oxide-library/src/adapters/local_git/primitives.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [primitives](/crates/oxide-library/src/adapters/local_git/primitives.md) |
