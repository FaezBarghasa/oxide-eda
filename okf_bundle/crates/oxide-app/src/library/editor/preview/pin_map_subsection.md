---
okf_version: "0.2"
type: Function
title: pin_map_subsection
resource: crates/oxide-app/src/library/editor/preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/preview/pin_map_subsection
language: rust
---

# pin_map_subsection

## Signature

```rust
fn pin_map_subsection(
    state: &'a ComponentPreviewState,
    tokens: &'a ThemeTokens,
    address: &EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 308–420 in `crates/oxide-app/src/library/editor/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/library/editor/preview.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [override_editor_row](/crates/oxide-app/src/library/editor/preview/override_editor_row.md) |
