---
okf_version: "0.2"
type: Function
title: view_constraint_submenu
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector/view_constraint_submenu
language: rust
---

# view_constraint_submenu

## Signature

```rust
fn view_constraint_submenu(
    editor: &'a FootprintEditorState,
    text_c: Color,
    muted: Color,
    border: Color,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Decorators

- `expect(
    dead_code,
    reason = "v0.14.2 replaced this with sketch_mode::active_bar; kept as a doc-only reference"
)`

## Source
Lines 223–392 in `crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [inspector](/crates/oxide-app/src/library/editor/footprint/sketch_mode/inspector.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
