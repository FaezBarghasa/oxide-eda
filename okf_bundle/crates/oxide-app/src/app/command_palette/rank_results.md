---
okf_version: "0.2"
type: Function
title: rank_results
description: Filter the catalog by query and rank the survivors. Returns indices
resource: crates/oxide-app/src/app/command_palette.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/command_palette/rank_results
language: rust
---

# rank_results

Filter the catalog by query and rank the survivors. Returns indices

## Signature

```rust
pub fn rank_results(catalog: &[CommandEntry], query: &str) -> Vec<(usize, i32)>
```

## Visibility

- `pub`

## Docstring

Filter the catalog by query and rank the survivors. Returns indices
into `catalog` paired with their score, descending by score (higher
is better). Empty query passes everything through with a baseline
rank that prefers Commands > Symbols > Files. The caller can
truncate to `MAX_RESULTS`.

## Source
Lines 216–257 in `crates/oxide-app/src/app/command_palette.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command_palette](/crates/oxide-app/src/app/command_palette.md) |
| calls | [fuzzy_score](/crates/oxide-app/src/app/command_palette/fuzzy_score.md) |
| called_by | [empty_query_passes_all](/crates/oxide-app/src/app/command_palette/empty_query_passes_all.md) |
| called_by | [rank_filters_and_orders](/crates/oxide-app/src/app/command_palette/rank_filters_and_orders.md) |
| called_by | [execute_command_palette_selected](/crates/oxide-app/src/app/dispatch/command_palette/execute_command_palette_selected.md) |
| called_by | [move_command_palette_selection](/crates/oxide-app/src/app/dispatch/command_palette/move_command_palette_selection.md) |
| called_by | [view_command_palette_dropdown](/crates/oxide-app/src/app/view/overlays/mod/view_command_palette_dropdown.md) |
