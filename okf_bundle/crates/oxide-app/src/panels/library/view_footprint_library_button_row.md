---
okf_version: "0.2"
type: Function
title: view_footprint_library_button_row
description: "Bottom button row for the Footprint Library panel — Altium's"
resource: crates/oxide-app/src/panels/library.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/library/view_footprint_library_button_row
language: rust
---

# view_footprint_library_button_row

Bottom button row for the Footprint Library panel — Altium's

## Signature

```rust
fn view_footprint_library_button_row(
    ctx: &'a PanelContext,
    selected: Option<usize>,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Docstring

Bottom button row for the Footprint Library panel — Altium's
`Place / Add / Delete / Edit` quartet.

## Source
Lines 444–506 in `crates/oxide-app/src/panels/library.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library](/crates/oxide-app/src/panels/library.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| called_by | [view_footprint_library](/crates/oxide-app/src/panels/library/view_footprint_library.md) |
