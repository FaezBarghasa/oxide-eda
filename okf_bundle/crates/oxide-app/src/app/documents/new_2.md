---
okf_version: "0.2"
type: Function
title: new
description: "Build a fresh standalone editor state from a `SymbolFile`"
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/new_2
language: rust
---

# new

Build a fresh standalone editor state from a `SymbolFile`

## Signature

```rust
impl SymbolEditorState { pub fn new(path: PathBuf, file: oxide_library::SymbolFile) -> Self }
```

## Visibility

- `pub`

## Docstring

Build a fresh standalone editor state from a `SymbolFile`
container loaded off disk. `path` is the `.snxsym` file the
user opened. The editor opens on the first symbol in the file.

## Source
Lines 330–357 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
