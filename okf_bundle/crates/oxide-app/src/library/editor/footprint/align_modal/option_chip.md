---
okf_version: "0.2"
type: Function
title: option_chip
description: "One mutually-exclusive option chip. `selected` fills it with the"
resource: crates/oxide-app/src/library/editor/footprint/align_modal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/align_modal/option_chip
language: rust
---

# option_chip

One mutually-exclusive option chip. `selected` fills it with the

## Signature

```rust
fn option_chip(
    label: &'a str,
    selected: bool,
    message: LibraryMessage,
    accent: Color,
    text_muted: Color,
    border: Color,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

One mutually-exclusive option chip. `selected` fills it with the
accent colour; otherwise it is a bordered, muted, transparent chip.

## Source
Lines 186–215 in `crates/oxide-app/src/library/editor/footprint/align_modal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [align_modal](/crates/oxide-app/src/library/editor/footprint/align_modal.md) |
| called_by | [view_align_modal](/crates/oxide-app/src/library/editor/footprint/align_modal/view_align_modal.md) |
