---
okf_version: "0.2"
type: Function
title: drc_marker_kind
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/drc_marker_kind
language: rust
---

# drc_marker_kind

## Signature

```rust
pub(super) fn drc_marker_kind(marker: &DrcMarkerInput) -> DrcMarkerKind
```

## Visibility

- `pub(super)`

## Source
Lines 295–320 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| called_by | [drc_marker_lines](/crates/oxide-renderer/src/pcb/emit/drc_marker_lines.md) |
| called_by | [drc_marker_vertices](/crates/oxide-renderer/src/pcb/emit/drc_marker_vertices.md) |
