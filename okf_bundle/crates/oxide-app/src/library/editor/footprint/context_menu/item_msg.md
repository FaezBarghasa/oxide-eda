---
okf_version: "0.2"
type: Function
title: item_msg
description: ── helpers ─────────────────────────────────────────────────────────
resource: crates/oxide-app/src/library/editor/footprint/context_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/context_menu/item_msg
language: rust
---

# item_msg

── helpers ─────────────────────────────────────────────────────────

## Signature

```rust
fn item_msg(
    tokens: &'a ThemeTokens,
    label: &str,
    shortcut: &str,
    message: LibraryMessage,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

── helpers ─────────────────────────────────────────────────────────

## Source
Lines 354–391 in `crates/oxide-app/src/library/editor/footprint/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/footprint/context_menu.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| called_by | [view_context_menu](/crates/oxide-app/src/library/editor/footprint/context_menu/view_context_menu.md) |
