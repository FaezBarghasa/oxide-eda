---
okf_version: "0.2"
type: Module
title: command_palette
description: Command palette — fuzzy-search peek that fronts the chrome strip.
resource: crates/oxide-app/src/app/command_palette.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/command_palette
language: rust
---

# command_palette

Command palette — fuzzy-search peek that fronts the chrome strip.

## Docstring

Command palette — fuzzy-search peek that fronts the chrome strip.

VS Code-style Ctrl+Shift+P entry. Three sources feed one flat catalog:

1. **Commands** — every registry command the bridge can resolve
(#366), plus every panel open. Rows carry the command id, not a
`Message`, and show the binding the active keymap profile gives
them (#374).
2. **Symbols** — placed designators in the active project (zoom-to).
3. **Files** — sheets/PCB/libraries in every loaded project (open).

Result list is capped at [`MAX_RESULTS`]; a "More results" footer
shows when more match. Sublime-text-style non-contiguous scoring with
word-boundary bonuses, contiguous-run bonuses, and a length penalty.

## Relationships

| Type | Target |
|------|--------|
| related | [CommandPaletteState](/crates/oxide-app/src/app/command_palette/CommandPaletteState.md) |
| related | [CommandSource](/crates/oxide-app/src/app/command_palette/CommandSource.md) |
| related | [CommandEntry](/crates/oxide-app/src/app/command_palette/CommandEntry.md) |
| related | [CommandAction](/crates/oxide-app/src/app/command_palette/CommandAction.md) |
| related | [build_catalog](/crates/oxide-app/src/app/command_palette/build_catalog.md) |
| related | [rank_results](/crates/oxide-app/src/app/command_palette/rank_results.md) |
| related | [fuzzy_score](/crates/oxide-app/src/app/command_palette/fuzzy_score.md) |
| related | [empty_query_passes_all](/crates/oxide-app/src/app/command_palette/empty_query_passes_all.md) |
| related | [every_palette_command_row_resolves_through_the_bridge](/crates/oxide-app/src/app/command_palette/every_palette_command_row_resolves_through_the_bridge.md) |
| related | [the_palette_offers_command_rows_from_the_catalog](/crates/oxide-app/src/app/command_palette/the_palette_offers_command_rows_from_the_catalog.md) |
| related | [a_bound_command_row_shows_its_shortcut](/crates/oxide-app/src/app/command_palette/a_bound_command_row_shows_its_shortcut.md) |
| related | [missing_chars_score_none](/crates/oxide-app/src/app/command_palette/missing_chars_score_none.md) |
| related | [prefix_outscores_substring](/crates/oxide-app/src/app/command_palette/prefix_outscores_substring.md) |
| related | [contiguous_outscores_split](/crates/oxide-app/src/app/command_palette/contiguous_outscores_split.md) |
| related | [word_boundary_bonus](/crates/oxide-app/src/app/command_palette/word_boundary_bonus.md) |
| related | [rank_filters_and_orders](/crates/oxide-app/src/app/command_palette/rank_filters_and_orders.md) |
