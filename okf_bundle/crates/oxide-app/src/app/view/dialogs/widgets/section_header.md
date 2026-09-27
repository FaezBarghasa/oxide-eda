---
okf_version: "0.2"
type: Function
title: section_header
description: "Subtle section divider used inside the BOM modal's properties"
resource: crates/oxide-app/src/app/view/dialogs/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/widgets/section_header
language: rust
---

# section_header

Subtle section divider used inside the BOM modal's properties

## Signature

```rust
pub(super) fn section_header(title: &str, muted: Color) -> Element<'_, Message>
```

## Visibility

- `pub(super)`

## Docstring

Subtle section divider used inside the BOM modal's properties
sidebar. Rendered as a 1-line label on a slightly tinted strip
so the panel reads as a stack of named sections.

## Source
Lines 176–192 in `crates/oxide-app/src/app/view/dialogs/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/app/view/dialogs/widgets.md) |
