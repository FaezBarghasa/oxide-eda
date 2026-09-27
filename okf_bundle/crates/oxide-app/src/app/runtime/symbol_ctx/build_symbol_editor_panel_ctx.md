---
okf_version: "0.2"
type: Function
title: build_symbol_editor_panel_ctx
description: Scan a library directory for standalone primitive files. Returns
resource: crates/oxide-app/src/app/runtime/symbol_ctx.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/symbol_ctx/build_symbol_editor_panel_ctx
language: rust
---

# build_symbol_editor_panel_ctx

Scan a library directory for standalone primitive files. Returns

## Signature

```rust
pub(super) fn build_symbol_editor_panel_ctx(
    app: &super::super::Oxide,
) -> Option<crate::panels::SymbolEditorPanelContext>
```

## Visibility

- `pub(super)`

## Docstring

Scan a library directory for standalone primitive files. Returns
`(symbols, footprints, sims)` triples — each `(stem, absolute_path)`.
Missing subdirectories are silently treated as empty so a fresh
library doesn't error; non-UTF-8 filenames and dotfiles are skipped.

Order is filename-stem-sorted so the project tree stays stable
across sessions (read_dir order is platform-dependent on Windows).
Project the active `.snxsym` editor's data into a panel-side
snapshot. Called from `refresh_panel_ctx` so the right-dock
Properties panel and the SCH-Library left-dock panel can render
context-aware content while the active tab is a Symbol editor.
Returns `None` for any other tab kind.

## Source
Lines 13–159 in `crates/oxide-app/src/app/runtime/symbol_ctx.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_ctx](/crates/oxide-app/src/app/runtime/symbol_ctx.md) |
| calls | [graphic_kind_to_summary](/crates/oxide-app/src/app/runtime/symbol_ctx/graphic_kind_to_summary.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
| calls | [max_part_number](/crates/oxide-app/src/library/editor/symbol/state/mod/max_part_number.md) |
| called_by | [refresh_panel_ctx](/crates/oxide-app/src/app/runtime/panel_ctx/refresh_panel_ctx.md) |
