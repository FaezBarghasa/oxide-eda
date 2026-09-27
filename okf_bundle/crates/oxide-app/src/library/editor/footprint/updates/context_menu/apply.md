---
okf_version: "0.2"
type: Function
title: apply
resource: crates/oxide-app/src/library/editor/footprint/updates/context_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/context_menu/apply
language: rust
---

# apply

## Signature

```rust
pub(super) fn apply(editor: &mut crate::app::FootprintEditorState, msg: FootprintEditorMsg)
```

## Visibility

- `pub(super)`

## Source
Lines 9–17 in `crates/oxide-app/src/library/editor/footprint/updates/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/footprint/updates/context_menu.md) |
| calls | [show](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/show.md) |
| calls | [close](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/close.md) |
| calls | [open_submenu](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/open_submenu.md) |
| calls | [run_action](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/run_action.md) |
