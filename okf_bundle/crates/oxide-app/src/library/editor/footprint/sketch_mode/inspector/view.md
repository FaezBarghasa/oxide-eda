---
okf_version: "0.2"
type: Function
title: view
description: "Render the inspector strip. Returns an empty `Space` when not in"
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view
language: rust
---

# view

Render the inspector strip. Returns an empty `Space` when not in

## Signature

```rust
pub fn view(
    editor: &'a FootprintEditorState,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render the inspector strip. Returns an empty `Space` when not in
Sketch mode so the caller can unconditionally add it to the body
column without `if`-branching.

## Source
Lines 33–90 in `crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [inspector](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [view_dof](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view_dof.md) |
| calls | [view_params](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view_params.md) |
| calls | [view_role](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view_role.md) |
| calls | [view_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view_warnings.md) |
