---
okf_version: "0.2"
type: Function
title: handle_symbol_primitive_edit
description: "Symbol-tab branch of [`Self::handle_primitive_editor_event`]."
resource: crates/oxide-app/src/app/dispatch/library/editor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/editor/handle_symbol_primitive_edit
language: rust
---

# handle_symbol_primitive_edit

Symbol-tab branch of [`Self::handle_primitive_editor_event`].

## Signature

```rust
impl Oxide { fn handle_symbol_primitive_edit(
        &mut self,
        path: std::path::PathBuf,
        msg: SymbolEditorMsg,
    ) -> Task<Message> }
```

## Docstring

Symbol-tab branch of [`Self::handle_primitive_editor_event`].
Per-library display settings (sheet color, grid, unit) mutate the
shared `OpenLibrary.display`; everything else routes to the
standalone symbol editor keyed by `path`.

## Source
Lines 239–316 in `crates/oxide-app/src/app/dispatch/library/editor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/app/dispatch/library/editor.md) |
| calls | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
