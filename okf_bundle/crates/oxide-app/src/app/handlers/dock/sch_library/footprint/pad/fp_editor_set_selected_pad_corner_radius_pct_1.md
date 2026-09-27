---
okf_version: "0.2"
type: Function
title: fp_editor_set_selected_pad_corner_radius_pct
description: "#599 — sibling of [`Self::fp_editor_set_next_pad_corner_radius_pct`];"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_corner_radius_pct_1
language: rust
---

# fp_editor_set_selected_pad_corner_radius_pct

#599 — sibling of [`Self::fp_editor_set_next_pad_corner_radius_pct`];

## Signature

```rust
pub(crate) fn fp_editor_set_selected_pad_corner_radius_pct(
        &mut self,
        idx: usize,
        value: String,
    ) -> bool
```

## Visibility

- `pub(crate)`

## Docstring

#599 — sibling of [`Self::fp_editor_set_next_pad_corner_radius_pct`];
an unreadable buffer is refused rather than written as `None`.

## Source
Lines 493–510 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.md) |
| calls | [fp_parse_optional_number_in](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_parse_optional_number_in.md) |
| calls | [fp_resolve_optional_number](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_resolve_optional_number.md) |
