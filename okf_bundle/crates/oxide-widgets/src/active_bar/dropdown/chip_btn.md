---
okf_version: "0.2"
type: Function
title: chip_btn
description: "Altium-style toggle chip — used inside `DropdownEntry::Custom`"
resource: crates/oxide-widgets/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/active_bar/dropdown/chip_btn
language: rust
---

# chip_btn

Altium-style toggle chip — used inside `DropdownEntry::Custom`

## Signature

```rust
pub fn chip_btn(
    label: impl Into<String>,
    on_press: M,
    enabled: bool,
    accent: Color,
) -> Element<'static, M>
```

## Type Parameters

- `M`

## Visibility

- `pub`

## Docstring

Altium-style toggle chip — used inside `DropdownEntry::Custom`
to build chip-wrap layouts (Selection Filter pill grids in the
schematic / footprint / future PCB editors).

`enabled = true` paints the chip with the accent border + active
background; `false` shows the muted inactive treatment. Click
fires `on_press`.

Caller composes a `Wrap` or `column![row![...], row![...]]`
from these chips and feeds the result into
`DropdownEntry::Custom(...)` so the same chrome lights up in
every editor.

## Source
Lines 294–331 in `crates/oxide-widgets/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-widgets/src/active_bar/dropdown.md) |
| called_by | [filter_entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/filter_entries.md) |
| called_by | [filter_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/filter_entries.md) |
