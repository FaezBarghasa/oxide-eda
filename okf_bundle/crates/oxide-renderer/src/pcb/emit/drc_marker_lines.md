---
okf_version: "0.2"
type: Function
title: drc_marker_lines
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/drc_marker_lines
language: rust
---

# drc_marker_lines

## Signature

```rust
pub(super) fn drc_marker_lines(marker: &DrcMarkerInput) -> Vec<([f32; 2], [f32; 2])>
```

## Visibility

- `pub(super)`

## Source
Lines 374–394 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| calls | [drc_marker_kind](/crates/oxide-renderer/src/pcb/emit/drc_marker_kind.md) |
| called_by | [emit_overlays](/crates/oxide-renderer/src/pcb/emit/emit_overlays.md) |
