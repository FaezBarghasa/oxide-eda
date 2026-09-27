---
okf_version: "0.2"
type: Function
title: template_row
description: Layout one slot row from the template — value editor + unit suffix +
resource: crates/oxide-app/src/library/editor/params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/params/template_row
language: rust
---

# template_row

Layout one slot row from the template — value editor + unit suffix +

## Signature

```rust
fn template_row(
    state: &'a ComponentPreviewState,
    slot: &'a ParamSlot,
    required: bool,
    tokens: &'a ThemeTokens,
    address: &EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

Layout one slot row from the template — value editor + unit suffix +
optional "✗ missing" badge for required-but-empty required slots.

## Source
Lines 152–198 in `crates/oxide-app/src/library/editor/params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [params](/crates/oxide-app/src/library/editor/params.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [slot_input](/crates/oxide-app/src/library/editor/params/slot_input.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/params/view.md) |
