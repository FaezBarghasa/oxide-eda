---
okf_version: "0.2"
type: Function
title: with_pads
description: "Internal constructor shared by `from_footprint` + `empty`."
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/with_pads_1
language: rust
---

# with_pads

Internal constructor shared by `from_footprint` + `empty`.

## Signature

```rust
fn with_pads(pads: Vec<EditorPad>) -> Self
```

## Docstring

Internal constructor shared by `from_footprint` + `empty`.
Centralising the field-by-field defaulting kills the giant
duplicated builder block that used to live in both call sites.

## Source
Lines 284–340 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |
