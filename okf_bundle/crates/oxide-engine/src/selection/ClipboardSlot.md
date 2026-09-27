---
okf_version: "0.2"
type: Class
title: ClipboardSlot
description: "Which `ClipboardSelection` field a kind is copied into by"
resource: crates/oxide-engine/src/selection.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/selection/ClipboardSlot
language: rust
---

# ClipboardSlot

Which `ClipboardSelection` field a kind is copied into by

## Signature

```rust
enum ClipboardSlot
```

## Docstring

Which `ClipboardSelection` field a kind is copied into by
`collect_selection_clipboard` (above). The match is exhaustive over
`SelectedKind` (no `_` arm), so a new variant is a compile error here
until it's classified — this is the one place that decides what
clipboard-based Copy/Cut/Paste can carry; `clipboard_can_carry` and
`collect_selection_clipboard` both derive from it instead of keeping
their own list.

## Source
Lines 591–599 in `crates/oxide-engine/src/selection.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [selection](/crates/oxide-engine/src/selection.md) |
