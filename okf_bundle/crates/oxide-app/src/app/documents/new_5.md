---
okf_version: "0.2"
type: Function
title: new
description: "Build a fresh standalone editor state from a `FootprintFile`"
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/new_5
language: rust
---

# new

Build a fresh standalone editor state from a `FootprintFile`

## Signature

```rust
pub fn new(path: PathBuf, file: oxide_library::FootprintFile) -> Self
```

## Visibility

- `pub`

## Docstring

Build a fresh standalone editor state from a `FootprintFile`
container loaded off disk. `path` is the `.snxfpt` file the
user opened. The editor opens on the first footprint in the
file. The caller is responsible for confirming the file is
non-empty before this call.

## Source
Lines 464–480 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
