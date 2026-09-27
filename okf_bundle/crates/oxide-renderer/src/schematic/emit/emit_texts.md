---
okf_version: "0.2"
type: Function
title: emit_texts
resource: crates/oxide-renderer/src/schematic/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/schematic/emit/emit_texts
language: rust
---

# emit_texts

## Signature

```rust
pub(super) fn emit_texts(snapshot: &SchematicSnapshot, scene: &mut Scene)
```

## Visibility

- `pub(super)`

## Source
Lines 111–124 in `crates/oxide-renderer/src/schematic/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/schematic/emit.md) |
| calls | [emit_text_bucket](/crates/oxide-renderer/src/schematic/emit/emit_text_bucket.md) |
| called_by | [build_scene](/crates/oxide-renderer/src/schematic/mod/build_scene.md) |
