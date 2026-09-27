---
okf_version: "0.2"
type: Function
title: run_action
resource: crates/oxide-app/src/library/editor/footprint/updates/context_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/context_menu/run_action
language: rust
---

# run_action

## Signature

```rust
fn run_action(
    editor: &mut crate::app::FootprintEditorState,
    action: crate::library::editor::footprint::state::FootprintContextAction,
)
```

## Source
Lines 86–128 in `crates/oxide-app/src/library/editor/footprint/updates/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/footprint/updates/context_menu.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/apply.md) |
