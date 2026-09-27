# command_palette

## Classs

- [CommandAction](CommandAction.md) — [derive(Debug, Clone)]
- [CommandEntry](CommandEntry.md) — [derive(Debug, Clone)]
- [CommandPaletteState](CommandPaletteState.md) — [derive(Debug, Clone, Default)]
- [CommandSource](CommandSource.md) — [derive(Debug, Clone, Copy, PartialEq, Eq)]

## Functions

- [a_bound_command_row_shows_its_shortcut](a_bound_command_row_shows_its_shortcut.md) — #374 — a command the active profile binds shows that binding as
- [build_catalog](build_catalog.md) — Build the full catalog from the live app state. Cheap: O(menu) +
- [contiguous_outscores_split](contiguous_outscores_split.md) — [test]
- [empty_query_passes_all](empty_query_passes_all.md) — [test]
- [every_palette_command_row_resolves_through_the_bridge](every_palette_command_row_resolves_through_the_bridge.md) — #366 — every command row the palette offers must resolve through
- [fuzzy_score](fuzzy_score.md) — Sublime-text-style fuzzy score. Returns `None` if any query
- [missing_chars_score_none](missing_chars_score_none.md) — [test]
- [prefix_outscores_substring](prefix_outscores_substring.md) — [test]
- [rank_filters_and_orders](rank_filters_and_orders.md) — [test]
- [rank_results](rank_results.md) — Filter the catalog by query and rank the survivors. Returns indices
- [the_palette_offers_command_rows_from_the_catalog](the_palette_offers_command_rows_from_the_catalog.md) — #366 — the palette must actually be reading the catalog. A build
- [word_boundary_bonus](word_boundary_bonus.md) — [test]
