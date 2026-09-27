---
okf_version: "0.2"
type: Function
title: open_library
description: "Open the `*.snxlib/` at `root`, mounting the adapter under its"
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/open_library
language: rust
---

# open_library

Open the `*.snxlib/` at `root`, mounting the adapter under its

## Signature

```rust
impl LibraryState { pub fn open_library(&mut self, root: PathBuf) -> Result<(), LibraryError> }
```

## Visibility

- `pub`

## Docstring

Open the `*.snxlib/` at `root`, mounting the adapter under its
`library_id` on `set` and registering the display entry in
`open_libraries`. Idempotent.

Also primes the per-library `tables` cache by running
`list_tables` + `read_table` for every table the adapter
exposes. Read errors warn through `tracing` and the affected
entries are left empty — one bad table doesn't sink the open
flow.

## Source
Lines 114–163 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
