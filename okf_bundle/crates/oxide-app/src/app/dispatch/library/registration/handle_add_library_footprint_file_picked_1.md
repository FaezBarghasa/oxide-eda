---
okf_version: "0.2"
type: Function
title: handle_add_library_footprint_file_picked
description: F34 — Footprint counterpart to
resource: crates/oxide-app/src/app/dispatch/library/registration.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/registration/handle_add_library_footprint_file_picked_1
language: rust
---

# handle_add_library_footprint_file_picked

F34 — Footprint counterpart to

## Signature

```rust
pub(crate) fn handle_add_library_footprint_file_picked(
        &mut self,
        path: std::path::PathBuf,
    ) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

F34 — Footprint counterpart to
[`handle_add_library_symbol_file_picked`]. Writes an empty
`FootprintFile` (TOML+TSV envelope), registers the file as a
project library entry, opens the file as a clean primitive
editor tab.

## Source
Lines 323–369 in `crates/oxide-app/src/app/dispatch/library/registration.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [registration](/crates/oxide-app/src/app/dispatch/library/registration.md) |
| calls | [atomic_write](/crates/oxide-app/src/app/dispatch/library/recovery/atomic_write.md) |
