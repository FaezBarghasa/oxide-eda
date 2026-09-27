---
okf_version: "0.2"
type: Function
title: handle_fp_editor_edit_pad_shape_param
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch/handle_fp_editor_edit_pad_shape_param
language: rust
---

# handle_fp_editor_edit_pad_shape_param

## Signature

```rust
impl Oxide { pub(in crate::app::handlers::dock::sch_library) fn handle_fp_editor_edit_pad_shape_param(
        &mut self,
        pad_idx: &usize,
        key: &str,
        value: &str,
    ) -> Task<Message> }
```

## Visibility

- `pub(in crate::app::handlers::dock::sch_library)`

## Source
Lines 36–85 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/sketch.md) |
