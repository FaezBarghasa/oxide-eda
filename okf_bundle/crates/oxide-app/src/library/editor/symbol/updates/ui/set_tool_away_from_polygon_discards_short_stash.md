---
okf_version: "0.2"
type: Function
title: set_tool_away_from_polygon_discards_short_stash
description: "Switching away from `PlacePolygon` with < 3 vertices discards"
resource: crates/oxide-app/src/library/editor/symbol/updates/ui.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/ui/set_tool_away_from_polygon_discards_short_stash
language: rust
---

# set_tool_away_from_polygon_discards_short_stash

Switching away from `PlacePolygon` with < 3 vertices discards

## Signature

```rust
fn set_tool_away_from_polygon_discards_short_stash()
```

## Decorators

- `test`

## Docstring

Switching away from `PlacePolygon` with < 3 vertices discards
the stash — no graphic, no undo entry.
[test]

## Source
Lines 101–111 in `crates/oxide-app/src/library/editor/symbol/updates/ui.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ui](/crates/oxide-app/src/library/editor/symbol/updates/ui.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/ui/new_editor.md) |
| calls | [apply_symbol_ui](/crates/oxide-app/src/library/editor/symbol/updates/ui/apply_symbol_ui.md) |
