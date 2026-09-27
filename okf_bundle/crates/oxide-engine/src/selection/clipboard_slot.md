---
okf_version: "0.2"
type: Function
title: clipboard_slot
resource: crates/oxide-engine/src/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/selection/clipboard_slot
language: rust
---

# clipboard_slot

## Signature

```rust
fn clipboard_slot(kind: SelectedKind) -> Option<ClipboardSlot>
```

## Source
Lines 601–617 in `crates/oxide-engine/src/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-engine/src/selection.md) |
| called_by | [clipboard_can_carry](/crates/oxide-engine/src/selection/clipboard_can_carry.md) |
| called_by | [collect_selection_clipboard](/crates/oxide-engine/src/selection/collect_selection_clipboard.md) |
