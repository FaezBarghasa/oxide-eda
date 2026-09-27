---
okf_version: "0.2"
type: Function
title: move_trigger_left_click_arms_select_tool
description: "#426 — the Place/Move trigger's left-click used to emit a dead"
resource: crates/oxide-app/src/library/editor/symbol/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/mod/move_trigger_left_click_arms_select_tool
language: rust
---

# move_trigger_left_click_arms_select_tool

#426 — the Place/Move trigger's left-click used to emit a dead

## Signature

```rust
fn move_trigger_left_click_arms_select_tool()
```

## Decorators

- `test`

## Docstring

#426 — the Place/Move trigger's left-click used to emit a dead
`ActiveBarStub("Move")`. It now arms the Select tool directly,
same as the Select trigger next to it: a symbol has no separate
move tool, so Move IS drag-under-Select.
[test]

## Source
Lines 250–258 in `crates/oxide-app/src/library/editor/symbol/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/symbol/active_bar/mod.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/active_bar/mod/new_editor.md) |
| calls | [dropdown_trigger_items](/crates/oxide-app/src/library/editor/symbol/active_bar/mod/dropdown_trigger_items.md) |
