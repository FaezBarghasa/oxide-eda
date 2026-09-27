---
okf_version: "0.2"
type: Function
title: filter_entries
resource: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/filter_entries
language: rust
---

# filter_entries

## Signature

```rust
fn filter_entries(f: SymbolSelectionFilter, path: PathBuf) -> Vec<DropdownEntry<LibraryMessage>>
```

## Source
Lines 72–132 in `crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dropdowns](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns.md) |
| calls | [chip_btn](/crates/oxide-widgets/src/active_bar/dropdown/chip_btn.md) |
| called_by | [entries](/crates/oxide-app/src/library/editor/symbol/active_bar/dropdowns/entries.md) |
