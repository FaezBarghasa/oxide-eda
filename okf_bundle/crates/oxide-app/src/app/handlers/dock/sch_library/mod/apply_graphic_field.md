---
okf_version: "0.2"
type: Function
title: apply_graphic_field
resource: crates/oxide-app/src/app/handlers/dock/sch_library/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/mod/apply_graphic_field
language: rust
---

# apply_graphic_field

## Signature

```rust
fn apply_graphic_field(
    g: &mut oxide_library::SymbolGraphic,
    field: crate::panels::GraphicFieldId,
    value: f64,
)
```

## Source
Lines 779–847 in `crates/oxide-app/src/app/handlers/dock/sch_library/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sch_library](/crates/oxide-app/src/app/handlers/dock/sch_library/mod.md) |
| called_by | [arc_degree_edits_stay_in_range_and_preserve_sweep](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/arc_degree_edits_stay_in_range_and_preserve_sweep.md) |
| called_by | [handle_dock_sch_library_message](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/handle_dock_sch_library_message.md) |
