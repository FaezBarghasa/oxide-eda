---
okf_version: "0.2"
type: Function
title: emit_vias
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/emit_vias
language: rust
---

# emit_vias

## Signature

```rust
pub(super) fn emit_vias(snapshot: &PcbSnapshot, theme: &ResolvedTheme, scene: &mut Scene)
```

## Visibility

- `pub(super)`

## Source
Lines 210–227 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| called_by | [build_scene](/crates/oxide-renderer/src/pcb/mod/build_scene.md) |
