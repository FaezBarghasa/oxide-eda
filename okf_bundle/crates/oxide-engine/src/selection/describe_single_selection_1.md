---
okf_version: "0.2"
type: Function
title: describe_single_selection
resource: crates/oxide-engine/src/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/selection/describe_single_selection_1
language: rust
---

# describe_single_selection

## Signature

```rust
pub fn describe_single_selection(&self, items: &[SelectedItem]) -> Option<SelectionDetails>
```

## Visibility

- `pub`

## Source
Lines 181–577 in `crates/oxide-engine/src/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-engine/src/selection.md) |
| calls | [circumcircle](/crates/oxide-types/src/schematic/mod/circumcircle.md) |
