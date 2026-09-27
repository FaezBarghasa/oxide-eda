---
okf_version: "0.2"
type: Function
title: net_color_entries
description: "NetColor menu: seven colour swatches (each an irregular `Custom` row —"
resource: crates/oxide-app/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/active_bar/dropdown/net_color_entries
language: rust
---

# net_color_entries

NetColor menu: seven colour swatches (each an irregular `Custom` row —

## Signature

```rust
fn net_color_entries(
    tokens: &ThemeTokens,
    tid: ThemeId,
    sel: bool,
    nc: bool,
) -> Vec<DropdownEntry<ActiveBarMsg>>
```

## Docstring

NetColor menu: seven colour swatches (each an irregular `Custom` row —
a colour chip in place of an SVG icon), then the Custom / Clear rows
as ordinary items. The Clear rows grey out when no net carries a
custom colour (`requires_net_color`).

## Source
Lines 526–632 in `crates/oxide-app/src/active_bar/dropdown.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdown](/crates/oxide-app/src/active_bar/dropdown.md) |
| calls | [dd_btn_style_f](/crates/oxide-app/src/active_bar/dropdown/dd_btn_style_f.md) |
| called_by | [dropdown_entries](/crates/oxide-app/src/active_bar/dropdown/dropdown_entries.md) |
| called_by | [net_color_swatches_and_gated_clear_rows](/crates/oxide-app/src/active_bar/dropdown/net_color_swatches_and_gated_clear_rows.md) |
