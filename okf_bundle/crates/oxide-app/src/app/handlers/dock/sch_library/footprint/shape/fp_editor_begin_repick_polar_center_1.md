---
okf_version: "0.2"
type: Function
title: fp_editor_begin_repick_polar_center
description: v0.23 — Begin re-picking a polar centre. Sets
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape/fp_editor_begin_repick_polar_center_1
language: rust
---

# fp_editor_begin_repick_polar_center

v0.23 — Begin re-picking a polar centre. Sets

## Signature

```rust
pub(crate) fn fp_editor_begin_repick_polar_center(
        &mut self,
        array_id: oxide_sketch::array::ArrayId,
    ) -> bool
```

## Visibility

- `pub(crate)`

## Docstring

v0.23 — Begin re-picking a polar centre. Sets
`ToolPending::RepickPolarCenter` so the next sketch click on a
Point overwrites the array's `center`. The dispatcher in
[`crate::app::dispatch::library`] consumes the pending state and
resets to `Idle`.

## Source
Lines 531–542 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shape](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/shape.md) |
