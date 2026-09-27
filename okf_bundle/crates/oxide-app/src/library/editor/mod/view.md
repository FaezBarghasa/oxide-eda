---
okf_version: "0.2"
type: Function
title: view
description: Render a Component Preview surface — five tabs (Preview / Parameters
resource: crates/oxide-app/src/library/editor/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/mod/view
language: rust
---

# view

Render a Component Preview surface — five tabs (Preview / Parameters

## Signature

```rust
pub fn view(
    state: &'a ComponentPreviewState,
    library_state: &'a LibraryState,
    tokens: &'a ThemeTokens,
    address: EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render a Component Preview surface — five tabs (Preview / Parameters
/ Supply / Datasheet / Simulation), all read-only for Symbol+
Footprint. The active tab body fills the panel; the header strip
and footer give the row's identity + save controls.

## Source
Lines 43–59 in `crates/oxide-app/src/library/editor/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [editor](/crates/oxide-app/src/library/editor/mod.md) |
| calls | [view_header](/crates/oxide-app/src/library/editor/mod/view_header.md) |
| calls | [view_tabs](/crates/oxide-app/src/library/editor/mod/view_tabs.md) |
| calls | [view_active_tab](/crates/oxide-app/src/library/editor/mod/view_active_tab.md) |
| calls | [view_footer](/crates/oxide-app/src/library/editor/mod/view_footer.md) |
| called_by | [view_active_tab](/crates/oxide-app/src/library/editor/mod/view_active_tab.md) |
