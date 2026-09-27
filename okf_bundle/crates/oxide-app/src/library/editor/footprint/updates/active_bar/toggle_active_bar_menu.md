---
okf_version: "0.2"
type: Function
title: toggle_active_bar_menu
resource: crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/active_bar/toggle_active_bar_menu
language: rust
---

# toggle_active_bar_menu

## Signature

```rust
fn toggle_active_bar_menu(
    editor: &mut crate::app::FootprintEditorState,
    menu: crate::library::editor::footprint::state::FpActiveBarMenu,
)
```

## Source
Lines 52–60 in `crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/footprint/updates/active_bar.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/apply.md) |
