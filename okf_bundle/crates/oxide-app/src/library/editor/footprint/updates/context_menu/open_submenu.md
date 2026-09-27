---
okf_version: "0.2"
type: Function
title: open_submenu
resource: crates/oxide-app/src/library/editor/footprint/updates/context_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/context_menu/open_submenu
language: rust
---

# open_submenu

## Signature

```rust
fn open_submenu(
    editor: &mut crate::app::FootprintEditorState,
    sm: Option<crate::library::editor::footprint::state::FootprintContextSubmenu>,
)
```

## Source
Lines 77–84 in `crates/oxide-app/src/library/editor/footprint/updates/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/footprint/updates/context_menu.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/apply.md) |
