---
okf_version: "0.2"
type: Function
title: fp_editor_set_selected_pad_side
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_side_1
language: rust
---

# fp_editor_set_selected_pad_side

## Signature

```rust
pub(crate) fn fp_editor_set_selected_pad_side(
        &mut self,
        idx: usize,
        side: crate::library::editor::footprint::state::PadSide,
    ) -> bool
```

## Visibility

- `pub(crate)`

## Source
Lines 408–434 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.md) |
