---
okf_version: "0.2"
type: Function
title: emit_wires
resource: crates/oxide-renderer/src/schematic/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/schematic/emit/emit_wires
language: rust
---

# emit_wires

## Signature

```rust
pub(super) fn emit_wires(snapshot: &SchematicSnapshot, theme: &ResolvedTheme, scene: &mut Scene)
```

## Visibility

- `pub(super)`

## Source
Lines 18–32 in `crates/oxide-renderer/src/schematic/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/schematic/emit.md) |
| calls | [resolve_wire_color](/crates/oxide-renderer/src/schematic/emit/resolve_wire_color.md) |
| called_by | [build_scene](/crates/oxide-renderer/src/schematic/mod/build_scene.md) |
