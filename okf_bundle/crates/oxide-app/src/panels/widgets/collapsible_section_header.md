---
okf_version: "0.2"
type: Function
title: collapsible_section_header
description: Just the header part of a collapsible section — clickable button
resource: crates/oxide-app/src/panels/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/widgets/collapsible_section_header
language: rust
---

# collapsible_section_header

Just the header part of a collapsible section — clickable button

## Signature

```rust
pub fn collapsible_section_header(
    key: &str,
    title: &str,
    collapsed: &CollapsedSections,
    header_color: Color,
    border_c: Color,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Just the header part of a collapsible section — clickable button
with SVG chevron + 1px rule. Returns whether the section is
collapsed via `is_collapsed_section(...)` so callers can guard
their body push without using a closure.

## Source
Lines 41–93 in `crates/oxide-app/src/panels/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/panels/widgets.md) |
| calls | [chevron_right](/crates/oxide-app/src/panels/widgets/chevron_right.md) |
| calls | [chevron_down](/crates/oxide-app/src/panels/widgets/chevron_down.md) |
| called_by | [props_section_header](/crates/oxide-app/src/panels/footprint_editor_properties/mod/props_section_header.md) |
| called_by | [collapsible_section](/crates/oxide-app/src/panels/widgets/collapsible_section.md) |
