---
okf_version: "0.2"
type: Function
title: id_slug
description: v0.24 Track A — UUID slug for parameter-name namespacing. Strips
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/id_slug
language: rust
---

# id_slug

v0.24 Track A — UUID slug for parameter-name namespacing. Strips

## Signature

```rust
pub(super) fn id_slug(id: SketchEntityId) -> String
```

## Visibility

- `pub(super)`

## Docstring

v0.24 Track A — UUID slug for parameter-name namespacing. Strips
dashes so the resulting parameter name is a valid identifier in
the expression language.

## Source
Lines 108–110 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.md) |
| called_by | [bind_shape_param](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/bind_shape_param.md) |
| called_by | [mirror_delete_pad_from_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_delete_pad_from_sketch.md) |
