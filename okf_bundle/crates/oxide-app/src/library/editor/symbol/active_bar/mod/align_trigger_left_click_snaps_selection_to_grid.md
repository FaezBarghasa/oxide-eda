---
okf_version: "0.2"
type: Function
title: align_trigger_left_click_snaps_selection_to_grid
description: "#426 — the Align trigger's left-click used to emit a dead"
resource: crates/oxide-app/src/library/editor/symbol/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/mod/align_trigger_left_click_snaps_selection_to_grid
language: rust
---

# align_trigger_left_click_snaps_selection_to_grid

#426 — the Align trigger's left-click used to emit a dead

## Signature

```rust
fn align_trigger_left_click_snaps_selection_to_grid()
```

## Decorators

- `test`

## Docstring

#426 — the Align trigger's left-click used to emit a dead
`ActiveBarStub("Align To Grid")`. It now dispatches the real
`AlignSelectedToGrid` snap.
[test]

## Source
Lines 264–272 in `crates/oxide-app/src/library/editor/symbol/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/symbol/active_bar/mod.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/active_bar/mod/new_editor.md) |
| calls | [dropdown_trigger_items](/crates/oxide-app/src/library/editor/symbol/active_bar/mod/dropdown_trigger_items.md) |
