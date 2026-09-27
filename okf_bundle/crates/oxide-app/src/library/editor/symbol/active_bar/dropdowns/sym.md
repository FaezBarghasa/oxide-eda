---
okf_version: "0.2"
type: Function
title: sym
description: "Convenience: route a `SymbolEditorMsg` to the editor at `path`."
resource: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/sym
language: rust
---

# sym

Convenience: route a `SymbolEditorMsg` to the editor at `path`.

## Signature

```rust
fn sym(path: PathBuf, msg: SymbolEditorMsg) -> LibraryMessage
```

## Docstring

Convenience: route a `SymbolEditorMsg` to the editor at `path`.

## Source
Lines 28–33 in `crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdowns](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.md) |
| called_by | [shapes_entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/shapes_entries.md) |
| called_by | [stub](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/stub.md) |
| called_by | [stub_with_icon](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/stub_with_icon.md) |
