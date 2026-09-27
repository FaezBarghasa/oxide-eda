---
okf_version: "0.2"
type: Function
title: place_move_row_arms_select_tool
description: "#426 — the Place dropdown's \"Move\" row used to be a dead"
resource: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/place_move_row_arms_select_tool
language: rust
---

# place_move_row_arms_select_tool

#426 — the Place dropdown's "Move" row used to be a dead

## Signature

```rust
fn place_move_row_arms_select_tool()
```

## Decorators

- `test`

## Docstring

#426 — the Place dropdown's "Move" row used to be a dead
`ActiveBarStub("Move")`. It now arms the Select tool, same
decision recorded on `place_entries`'s doc comment.
[test]

## Source
Lines 447–453 in `crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdowns](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.md) |
| calls | [place_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/place_entries.md) |
