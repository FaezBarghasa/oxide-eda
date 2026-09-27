---
okf_version: "0.2"
type: Function
title: show
description: v0.26 — right-click context menu plumbing. State-only
resource: crates/oxide-app/src/library/editor/footprint/updates/context_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/context_menu/show
language: rust
---

# show

v0.26 — right-click context menu plumbing. State-only

## Signature

```rust
fn show(
    editor: &mut crate::app::FootprintEditorState,
    x: f32,
    y: f32,
    target: crate::library::editor::footprint::state::FootprintContextTarget,
)
```

## Docstring

v0.26 — right-click context menu plumbing. State-only
mutations; canvas cache is cleared when target adjusts the
selection (right-click on a pad selects it Altium-style).

## Source
Lines 22–71 in `crates/oxide-app/src/library/editor/footprint/updates/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/footprint/updates/context_menu.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/context_menu/apply.md) |
| called_by | [build_text_only_pdf](/crates/oxide-library/tests/ai_stub/build_text_only_pdf.md) |
