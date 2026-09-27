---
okf_version: "0.2"
type: Function
title: emit_text_bucket
resource: crates/oxide-renderer/src/schematic/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/schematic/emit/emit_text_bucket
language: rust
---

# emit_text_bucket

## Signature

```rust
pub(super) fn emit_text_bucket(texts: &[TextInput], output: &mut Vec<TextItem>)
```

## Visibility

- `pub(super)`

## Source
Lines 79–93 in `crates/oxide-renderer/src/schematic/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/schematic/emit.md) |
| calls | [to_text_h_align](/crates/oxide-renderer/src/schematic/emit/to_text_h_align.md) |
| calls | [to_text_v_align](/crates/oxide-renderer/src/schematic/emit/to_text_v_align.md) |
| called_by | [emit_texts](/crates/oxide-renderer/src/schematic/emit/emit_texts.md) |
