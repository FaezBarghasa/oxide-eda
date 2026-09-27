---
okf_version: "0.2"
type: Function
title: resolve_wire_color
resource: crates/oxide-renderer/src/schematic/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/schematic/emit/resolve_wire_color
language: rust
---

# resolve_wire_color

## Signature

```rust
pub(super) fn resolve_wire_color(
    wire: &WireInput,
    snapshot: &SchematicSnapshot,
    theme: &ResolvedTheme,
) -> [f32; 4]
```

## Visibility

- `pub(super)`

## Source
Lines 5–16 in `crates/oxide-renderer/src/schematic/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/schematic/emit.md) |
| called_by | [emit_wires](/crates/oxide-renderer/src/schematic/emit/emit_wires.md) |
