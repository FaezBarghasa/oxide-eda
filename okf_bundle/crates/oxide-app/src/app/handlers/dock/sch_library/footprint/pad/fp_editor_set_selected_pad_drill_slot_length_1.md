---
okf_version: "0.2"
type: Function
title: fp_editor_set_selected_pad_drill_slot_length
description: "v0.20 placeholder — slot length is not yet stored per selected pad,"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_drill_slot_length_1
language: rust
---

# fp_editor_set_selected_pad_drill_slot_length

v0.20 placeholder — slot length is not yet stored per selected pad,

## Signature

```rust
pub(crate) fn fp_editor_set_selected_pad_drill_slot_length(
        &mut self,
        _idx: usize,
        _value: String,
    ) -> bool
```

## Visibility

- `pub(crate)`

## Docstring

v0.20 placeholder — slot length is not yet stored per selected pad,
only on `NextPadDefaults`, so there is nothing for `_idx` to address
and nothing to parse out of `_value`. Both regain their names when
`EditorPad` gains its own `drill_slot_length_mm` (the selected-pad
follow-up the pad form notes for v0.21). Kept at the sibling
`(idx, value)` shape so the dispatch arm in `sch_library/mod.rs`
stays uniform with every other selected-pad setter.

## Source
Lines 483–490 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.md) |
