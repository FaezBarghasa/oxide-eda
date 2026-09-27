---
okf_version: "0.2"
type: Function
title: enabled_kinds
description: "v0.14 — enabled kinds in canonical `SelectionFilterKind::ALL`"
resource: crates/oxide-app/src/library/editor/footprint/state/selection_filter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/selection_filter/enabled_kinds_1
language: rust
---

# enabled_kinds

v0.14 — enabled kinds in canonical `SelectionFilterKind::ALL`

## Signature

```rust
pub fn enabled_kinds(&self) -> Vec<SelectionFilterKind>
```

## Visibility

- `pub`

## Docstring

v0.14 — enabled kinds in canonical `SelectionFilterKind::ALL`
order. Inverse of `apply_kinds`; used to capture a preset from
the current filter state.

## Source
Lines 222–228 in `crates/oxide-app/src/library/editor/footprint/state/selection_filter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection_filter](/crates/oxide-app/src/library/editor/footprint/state/selection_filter.md) |
