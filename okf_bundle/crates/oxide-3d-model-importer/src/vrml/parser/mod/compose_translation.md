---
okf_version: "0.2"
type: Function
title: compose_translation
resource: crates/oxide-3d-model-importer/src/vrml/parser/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/vrml/parser/mod/compose_translation
language: rust
---

# compose_translation

## Signature

```rust
impl Transform { fn compose_translation(
        parent: &Transform,
        translation: [f32; 3],
        scale: [f32; 3],
    ) -> Transform }
```

## Source
Lines 94–113 in `crates/oxide-3d-model-importer/src/vrml/parser/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parser](/crates/oxide-3d-model-importer/src/vrml/parser/mod.md) |
