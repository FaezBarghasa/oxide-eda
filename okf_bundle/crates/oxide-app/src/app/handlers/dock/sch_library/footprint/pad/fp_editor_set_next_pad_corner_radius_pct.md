---
okf_version: "0.2"
type: Function
title: fp_editor_set_next_pad_corner_radius_pct
description: "#599 — the three outcomes stay apart: empty clears, out of range"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_next_pad_corner_radius_pct
language: rust
---

# fp_editor_set_next_pad_corner_radius_pct

#599 — the three outcomes stay apart: empty clears, out of range

## Signature

```rust
impl Oxide { pub(crate) fn fp_editor_set_next_pad_corner_radius_pct(&mut self, value: String) -> bool }
```

## Visibility

- `pub(crate)`

## Docstring

#599 — the three outcomes stay apart: empty clears, out of range
clamps to the nearest bound, unreadable text leaves the stored
default alone. Collapsing them into one `None` used to erase the
value and drop the key from the saved `.snxfpt`.

## Source
Lines 175–189 in `crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad.md) |
| calls | [fp_parse_optional_number_in](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_parse_optional_number_in.md) |
| calls | [fp_resolve_optional_number](/crates/oxide-app/src/app/handlers/dock/sch_library/numeric_input/fp_resolve_optional_number.md) |
