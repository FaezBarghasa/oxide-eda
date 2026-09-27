---
okf_version: "0.2"
type: Function
title: handle_fp_editor_set_next_pad_hole_rotation
description: "#599 — an unreadable rotation leaves the stored default alone"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/handle_fp_editor_set_next_pad_hole_rotation_1
language: rust
---

# handle_fp_editor_set_next_pad_hole_rotation

#599 — an unreadable rotation leaves the stored default alone

## Signature

```rust
pub(in crate::app::handlers::dock::sch_library) fn handle_fp_editor_set_next_pad_hole_rotation(
        &mut self,
        v: &str,
    ) -> bool
```

## Visibility

- `pub(in crate::app::handlers::dock::sch_library)`

## Docstring

#599 — an unreadable rotation leaves the stored default alone
instead of clearing it.

## Source
Lines 725–738 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.md) |
| calls | [fp_parse_optional_number](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_parse_optional_number.md) |
| calls | [fp_resolve_optional_number](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_resolve_optional_number.md) |
