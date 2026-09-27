---
okf_version: "0.2"
type: Function
title: filter_entry
description: "Selection Filter menu: an irregular chip-wrap layout (All toggle +"
resource: crates/oxide-app/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/active_bar/dropdown/filter_entry
language: rust
---

# filter_entry

Selection Filter menu: an irregular chip-wrap layout (All toggle +

## Signature

```rust
fn filter_entry(
    tokens: &ThemeTokens,
    filters: &std::collections::HashSet<SelectionFilter>,
    custom_presets: &[CustomFilterPreset],
) -> DropdownEntry<ActiveBarMsg>
```

## Docstring

Selection Filter menu: an irregular chip-wrap layout (All toggle +
user presets, then two rows of six category chips) that can't be
expressed as a vertical `Item` list, so it rides the widget's
`Custom` escape hatch as a single owned `Element<'static>`.

## Source
Lines 638–819 in `crates/oxide-app/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-app/src/active_bar/dropdown.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [filter_menu_is_a_single_custom_entry](/crates/oxide-app/src/active_bar/dropdown/filter_menu_is_a_single_custom_entry.md) |
