---
okf_version: "0.2"
type: Function
title: catalog_labels_match_the_active_bar_literals
description: "Behaviour proof for #271: for every Active Bar row the catalog now"
resource: crates/oxide-app/src/app/command/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/command/active_bar/catalog_labels_match_the_active_bar_literals
language: rust
---

# catalog_labels_match_the_active_bar_literals

Behaviour proof for #271: for every Active Bar row the catalog now

## Signature

```rust
fn catalog_labels_match_the_active_bar_literals()
```

## Decorators

- `test`

## Docstring

Behaviour proof for #271: for every Active Bar row the catalog now
labels, the catalog's `menu_label` must equal the literal that used
to be rendered — so routing labels through the registry changes
nothing the user sees.

Scans `dropdown.rs`'s own source for the `(literal, action)` pairs
rather than trusting a hand-kept copy, which is the same reason
`bridge.rs` scans itself. If a row's wording is ever changed in the
view without the catalog following, this fails and names it.

Changing an Active Bar label is menu content and needs Caner's or
Hakan's sign-off; this test exists so such a change cannot happen
by accident inside a refactor.
[test]

## Source
Lines 323–393 in `crates/oxide-app/src/app/command/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/app/command/active_bar.md) |
| calls | [scan_dropdown_rows](/crates/oxide-app/src/app/command/active_bar/scan_dropdown_rows.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [metadata_for](/crates/oxide-app/src/keymap/catalog/mod/metadata_for.md) |
