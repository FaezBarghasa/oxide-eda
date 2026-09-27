---
okf_version: "0.2"
type: Function
title: apply_kinds
description: "v0.14 — enable exactly `kinds`, disable everything else. Applies"
resource: crates/oxide-app/src/library/editor/footprint/state/selection_filter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/selection_filter/apply_kinds
language: rust
---

# apply_kinds

v0.14 — enable exactly `kinds`, disable everything else. Applies

## Signature

```rust
impl SelectionFilter { pub fn apply_kinds(&mut self, kinds: &[SelectionFilterKind]) }
```

## Visibility

- `pub`

## Docstring

v0.14 — enable exactly `kinds`, disable everything else. Applies
a footprint filter preset (`FootprintFilterPreset`, Task 6).

## Source
Lines 212–217 in `crates/oxide-app/src/library/editor/footprint/state/selection_filter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection_filter](/crates/oxide-app/src/library/editor/footprint/state/selection_filter.md) |
