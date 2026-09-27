---
okf_version: "0.2"
type: Function
title: view
description: Render the active recovery dialog.
resource: crates/oxide-app/src/library/recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/recovery/view
language: rust
---

# view

Render the active recovery dialog.

## Signature

```rust
pub fn view(
    dialog: &'a RecoveryDialog,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render the active recovery dialog.

## Source
Lines 108–125 in `crates/oxide-app/src/library/recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [recovery](/crates/oxide-app/src/library/recovery.md) |
| calls | [library_missing_view](/crates/oxide-app/src/library/recovery/library_missing_view.md) |
| calls | [git_missing_view](/crates/oxide-app/src/library/recovery/git_missing_view.md) |
| calls | [broken_binding_view](/crates/oxide-app/src/library/recovery/broken_binding_view.md) |
