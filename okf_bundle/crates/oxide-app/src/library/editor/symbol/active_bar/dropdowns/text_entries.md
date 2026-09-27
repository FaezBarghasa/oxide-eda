---
okf_version: "0.2"
type: Function
title: text_entries
resource: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/text_entries
language: rust
---

# text_entries

## Signature

```rust
fn text_entries(
    active_tool: SymbolTool,
    path: PathBuf,
    tid: ThemeId,
) -> Vec<DropdownEntry<LibraryMessage>>
```

## Source
Lines 345–368 in `crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdowns](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.md) |
| called_by | [entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/entries.md) |
