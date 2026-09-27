---
okf_version: "0.2"
type: Function
title: text_entries
resource: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/text_entries
language: rust
---

# text_entries

## Signature

```rust
fn text_entries(
    state: &FootprintEditorState,
    path: PathBuf,
    tid: ThemeId,
) -> Vec<DropdownEntry<LibraryMessage>>
```

## Source
Lines 721–755 in `crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar_dropdowns](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns.md) |
| called_by | [entries](/crates/oxide-app/src/library/editor/footprint/active_bar_dropdowns/entries.md) |
