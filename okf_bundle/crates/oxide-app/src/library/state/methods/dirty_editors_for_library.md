---
okf_version: "0.2"
type: Function
title: dirty_editors_for_library
description: "Editor addresses currently pointing at `root` that have unsaved edits."
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/dirty_editors_for_library
language: rust
---

# dirty_editors_for_library

Editor addresses currently pointing at `root` that have unsaved edits.

## Signature

```rust
impl LibraryState { pub fn dirty_editors_for_library(&self, root: &Path) -> Vec<EditorAddress> }
```

## Visibility

- `pub`

## Docstring

Editor addresses currently pointing at `root` that have unsaved edits.

## Source
Lines 296–310 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
