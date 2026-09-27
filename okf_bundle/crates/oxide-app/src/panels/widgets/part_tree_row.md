---
okf_version: "0.2"
type: Function
title: part_tree_row
description: Render one indented part row inside the SCH Library tree-expander.
resource: crates/oxide-app/src/panels/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/widgets/part_tree_row
language: rust
---

# part_tree_row

Render one indented part row inside the SCH Library tree-expander.

## Signature

```rust
pub fn part_tree_row(
    label: &str,
    part: u8,
    is_active: bool,
    primary: Color,
    muted: Color,
    bg_active: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render one indented part row inside the SCH Library tree-expander.
Active part gets the selection background; otherwise the row hovers
like the symbol rows above it.

## Source
Lines 166–211 in `crates/oxide-app/src/panels/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/panels/widgets.md) |
| called_by | [view_sch_library](/crates/oxide-app/src/panels/library/view_sch_library.md) |
