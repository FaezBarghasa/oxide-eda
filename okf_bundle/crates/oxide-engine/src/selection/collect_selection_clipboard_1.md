---
okf_version: "0.2"
type: Function
title: collect_selection_clipboard
resource: crates/oxide-engine/src/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/selection/collect_selection_clipboard_1
language: rust
---

# collect_selection_clipboard

## Signature

```rust
pub fn collect_selection_clipboard(&self, items: &[SelectedItem]) -> ClipboardSelection
```

## Visibility

- `pub`

## Source
Lines 52–110 in `crates/oxide-engine/src/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-engine/src/selection.md) |
| calls | [clipboard_slot](/crates/oxide-engine/src/selection/clipboard_slot.md) |
