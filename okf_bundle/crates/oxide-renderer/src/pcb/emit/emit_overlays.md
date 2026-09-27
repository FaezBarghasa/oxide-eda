---
okf_version: "0.2"
type: Function
title: emit_overlays
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/emit_overlays
language: rust
---

# emit_overlays

## Signature

```rust
pub(super) fn emit_overlays(snapshot: &PcbSnapshot, theme: &ResolvedTheme, scene: &mut Scene)
```

## Visibility

- `pub(super)`

## Source
Lines 396–449 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| calls | [with_alpha_mul](/crates/oxide-renderer/src/pcb/emit/with_alpha_mul.md) |
| calls | [drc_slot](/crates/oxide-renderer/src/pcb/emit/drc_slot.md) |
| calls | [drc_marker_vertices](/crates/oxide-renderer/src/pcb/emit/drc_marker_vertices.md) |
| calls | [drc_marker_lines](/crates/oxide-renderer/src/pcb/emit/drc_marker_lines.md) |
