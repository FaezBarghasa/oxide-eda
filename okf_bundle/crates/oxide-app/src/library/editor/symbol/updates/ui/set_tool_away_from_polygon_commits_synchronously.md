---
okf_version: "0.2"
type: Function
title: set_tool_away_from_polygon_commits_synchronously
description: "Footprint parity: switching the tool away from `PlacePolygon`"
resource: crates/oxide-app/src/library/editor/symbol/updates/ui.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/ui/set_tool_away_from_polygon_commits_synchronously
language: rust
---

# set_tool_away_from_polygon_commits_synchronously

Footprint parity: switching the tool away from `PlacePolygon`

## Signature

```rust
fn set_tool_away_from_polygon_commits_synchronously()
```

## Decorators

- `test`

## Docstring

Footprint parity: switching the tool away from `PlacePolygon`
with >= 3 vertices collected commits synchronously, in the
same handler that changes `editor.tool` — no separate message
round-trip.
[test]

## Source
Lines 81–96 in `crates/oxide-app/src/library/editor/symbol/updates/ui.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ui](/crates/oxide-app/src/library/editor/symbol/updates/ui.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/ui/new_editor.md) |
| calls | [apply_symbol_ui](/crates/oxide-app/src/library/editor/symbol/updates/ui/apply_symbol_ui.md) |
