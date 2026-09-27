---
okf_version: "0.2"
type: Function
title: view_active_tab
resource: crates/oxide-app/src/library/editor/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/mod/view_active_tab
language: rust
---

# view_active_tab

## Signature

```rust
fn view_active_tab(
    state: &'a ComponentPreviewState,
    library_state: &'a LibraryState,
    tokens: &'a ThemeTokens,
    address: EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 131–149 in `crates/oxide-app/src/library/editor/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/library/editor/mod.md) |
| calls | [view](/crates/oxide-app/src/library/editor/mod/view.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/mod/view.md) |
