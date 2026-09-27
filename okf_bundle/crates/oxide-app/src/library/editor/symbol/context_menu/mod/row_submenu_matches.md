---
okf_version: "0.2"
type: Function
title: row_submenu_matches
description: "Whether `id`'s row is the currently accordion-open submenu. A"
resource: crates/oxide-app/src/library/editor/symbol/context_menu/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/context_menu/mod/row_submenu_matches
language: rust
---

# row_submenu_matches

Whether `id`'s row is the currently accordion-open submenu. A

## Signature

```rust
fn row_submenu_matches(id: &str, open: Option<SymbolContextSubmenu>) -> bool
```

## Docstring

Whether `id`'s row is the currently accordion-open submenu. A
plain `match` (not a lookup table) is fine — there's exactly one
submenu today; a second one adds one more arm here.

## Source
Lines 87–92 in `crates/oxide-app/src/library/editor/symbol/context_menu/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/symbol/context_menu/mod.md) |
| called_by | [flatten](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten.md) |
