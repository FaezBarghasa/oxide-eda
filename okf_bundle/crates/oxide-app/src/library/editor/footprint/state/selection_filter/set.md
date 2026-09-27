---
okf_version: "0.2"
type: Function
title: set
description: "v0.14 — set a single kind's flag. Mirrors `get`'s match arms."
resource: crates/oxide-app/src/library/editor/footprint/state/selection_filter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/selection_filter/set
language: rust
---

# set

v0.14 — set a single kind's flag. Mirrors `get`'s match arms.

## Signature

```rust
impl SelectionFilter { pub fn set(&mut self, kind: SelectionFilterKind, on: bool) }
```

## Visibility

- `pub`

## Docstring

v0.14 — set a single kind's flag. Mirrors `get`'s match arms.

## Source
Lines 193–208 in `crates/oxide-app/src/library/editor/footprint/state/selection_filter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection_filter](/crates/oxide-app/src/library/editor/footprint/state/selection_filter.md) |
