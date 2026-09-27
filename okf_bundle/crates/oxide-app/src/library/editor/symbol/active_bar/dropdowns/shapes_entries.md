---
okf_version: "0.2"
type: Function
title: shapes_entries
resource: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/shapes_entries
language: rust
---

# shapes_entries

## Signature

```rust
fn shapes_entries(
    active_tool: SymbolTool,
    path: PathBuf,
    tid: ThemeId,
) -> Vec<DropdownEntry<LibraryMessage>>
```

## Source
Lines 370–410 in `crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdowns](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.md) |
| calls | [sym](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/sym.md) |
| called_by | [entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/entries.md) |
