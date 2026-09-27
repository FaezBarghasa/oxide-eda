---
okf_version: "0.2"
type: Function
title: view_sch_library
description: ─── SCH Library Panel (Altium parity) ───────────────────────────────
resource: crates/oxide-app/src/panels/library.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/library/view_sch_library
language: rust
---

# view_sch_library

─── SCH Library Panel (Altium parity) ───────────────────────────────

## Signature

```rust
pub fn view_sch_library(ctx: &'a PanelContext) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

─── SCH Library Panel (Altium parity) ───────────────────────────────

When a `.snxsym` standalone editor tab is active, this panel lists
the symbols in the open `SymbolFile` container. Click switches the
active symbol; "Add Symbol" appends a fresh empty Symbol to the
container and makes it active. Read-only on Symbol metadata for now —
rename / designator-prefix edits go through the right-dock Properties
panel.

When no Symbol editor is open the panel renders a hint pointing the
user at the project tree's `Add New ▸ Symbol` flow.

## Source
Lines 57–300 in `crates/oxide-app/src/panels/library.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library](/crates/oxide-app/src/panels/library.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [part_tree_row](/crates/oxide-app/src/panels/widgets/part_tree_row.md) |
| called_by | [view_panel](/crates/oxide-app/src/panels/mod/view_panel.md) |
