---
okf_version: "0.2"
type: Function
title: drc_slot
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/drc_slot
language: rust
---

# drc_slot

## Signature

```rust
pub(super) fn drc_slot(severity: Severity) -> ColorSlot
```

## Visibility

- `pub(super)`

## Source
Lines 278–284 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| called_by | [emit_overlays](/crates/oxide-renderer/src/pcb/emit/emit_overlays.md) |
