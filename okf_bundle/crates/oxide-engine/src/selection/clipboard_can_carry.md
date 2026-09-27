---
okf_version: "0.2"
type: Function
title: clipboard_can_carry
resource: crates/oxide-engine/src/selection.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/selection/clipboard_can_carry
language: rust
---

# clipboard_can_carry

## Signature

```rust
fn clipboard_can_carry(kind: SelectedKind) -> bool
```

## Source
Lines 619–621 in `crates/oxide-engine/src/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-engine/src/selection.md) |
| calls | [clipboard_slot](/crates/oxide-engine/src/selection/clipboard_slot.md) |
| called_by | [clipboard_can_carry_matches_the_documented_seven_kinds](/crates/oxide-engine/src/selection/clipboard_can_carry_matches_the_documented_seven_kinds.md) |
| called_by | [partition_cuttable](/crates/oxide-engine/src/selection/partition_cuttable.md) |
