---
okf_version: "0.2"
type: Module
title: preview
description: "Library Browser — detail preview pane (DEAD CODE, F15 final pass)."
resource: crates/oxide-app/src/library/browser/preview.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/browser/preview
language: rust
---

# preview

Library Browser — detail preview pane (DEAD CODE, F15 final pass).

## Docstring

Library Browser — detail preview pane (DEAD CODE, F15 final pass).

`view_preview_pane` / `preview_panel` / `preview_panel_with_pick` /
`symbol_summary` / `footprint_summary` / `short_row_id` are kept as
dead code so the prune stays reviewable in one commit. The
Properties panel reads `PanelContext.library_row_detail` and renders
the equivalent. Pruning this block in the next cleanup pass.
Moved verbatim from the former single-file `browser` module.

## Relationships

| Type | Target |
|------|--------|
| related | [view_preview_pane](/crates/oxide-app/src/library/browser/preview/view_preview_pane.md) |
| related | [short_row_id](/crates/oxide-app/src/library/browser/preview/short_row_id.md) |
| related | [preview_panel](/crates/oxide-app/src/library/browser/preview/preview_panel.md) |
| related | [preview_panel_with_pick](/crates/oxide-app/src/library/browser/preview/preview_panel_with_pick.md) |
| related | [symbol_summary](/crates/oxide-app/src/library/browser/preview/symbol_summary.md) |
| related | [footprint_summary](/crates/oxide-app/src/library/browser/preview/footprint_summary.md) |
