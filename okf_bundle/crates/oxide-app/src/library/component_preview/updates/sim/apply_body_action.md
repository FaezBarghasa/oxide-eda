---
okf_version: "0.2"
type: Function
title: apply_body_action
description: "Apply a text-editor action to the simulation body, mirroring the new"
resource: crates/oxide-app/src/library/component_preview/updates/sim.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/sim/apply_body_action
language: rust
---

# apply_body_action

Apply a text-editor action to the simulation body, mirroring the new

## Signature

```rust
pub(super) fn apply_body_action(
    state: &mut ComponentPreviewState,
    action: iced::widget::text_editor::Action,
)
```

## Visibility

- `pub(super)`

## Docstring

Apply a text-editor action to the simulation body, mirroring the new
text back onto the model.

## Source
Lines 66–78 in `crates/oxide-app/src/library/component_preview/updates/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-app/src/library/component_preview/updates/sim.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
