---
okf_version: "0.2"
type: Function
title: as_footprint_editor
description: "`Some(path)` if this tab is a standalone Footprint editor."
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/as_footprint_editor
language: rust
---

# as_footprint_editor

`Some(path)` if this tab is a standalone Footprint editor.

## Signature

```rust
impl TabKind { pub fn as_footprint_editor(&self) -> Option<&PathBuf> }
```

## Visibility

- `pub`

## Docstring

`Some(path)` if this tab is a standalone Footprint editor.

## Source
Lines 74–79 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
