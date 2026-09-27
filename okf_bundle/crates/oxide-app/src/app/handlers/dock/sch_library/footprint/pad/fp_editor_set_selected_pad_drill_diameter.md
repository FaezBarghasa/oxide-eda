---
okf_version: "0.2"
type: Function
title: fp_editor_set_selected_pad_drill_diameter
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_drill_diameter
language: rust
---

# fp_editor_set_selected_pad_drill_diameter

## Signature

```rust
impl Oxide { pub(crate) fn fp_editor_set_selected_pad_drill_diameter(
        &mut self,
        idx: usize,
        value: String,
    ) -> bool }
```

## Visibility

- `pub(crate)`

## Source
Lines 467–475 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.md) |
| calls | [fp_parse_optional_mm](/crates/oxide-app/src/app/handlers/dock/sch_library/mod/fp_parse_optional_mm.md) |
