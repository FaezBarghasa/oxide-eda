---
okf_version: "0.2"
type: Function
title: escape_closes_context_menu_before_clearing_selection
description: "Esc used to have no effect on the footprint editor's right-click"
resource: crates/oxide-app/src/library/editor/footprint/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/tests/escape_closes_context_menu_before_clearing_selection
language: rust
---

# escape_closes_context_menu_before_clearing_selection

Esc used to have no effect on the footprint editor's right-click

## Signature

```rust
fn escape_closes_context_menu_before_clearing_selection()
```

## Decorators

- `test`

## Docstring

Esc used to have no effect on the footprint editor's right-click
menu — the only way out was clicking somewhere harmless. It now
closes the menu, and stops there, so backing out of a menu doesn't
also throw away the selection the menu was opened to act on.
[test]

## Source
Lines 219–247 in `crates/oxide-app/src/library/editor/footprint/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/tests.md) |
| calls | [default_editor](/crates/oxide-app/src/library/editor/footprint/tests/default_editor.md) |
