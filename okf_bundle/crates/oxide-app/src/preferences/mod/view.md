---
okf_version: "0.2"
type: Function
title: view
description: Build the full-screen backdrop + centred dialog.
resource: crates/oxide-app/src/preferences/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/preferences/mod/view
language: rust
---

# view

Build the full-screen backdrop + centred dialog.

## Signature

```rust
pub fn view(v: PrefsView<'a>) -> Element<'a, PrefMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Build the full-screen backdrop + centred dialog.

* `draft_theme`      — theme currently selected in the dialog (not yet saved)
* `saved_theme`      — the committed theme (used to detect unsaved changes)
* `draft_font`       — UI font name pending save
* `custom_name`      — name of the loaded custom theme (if any)
* `dirty`            — whether there are unsaved changes

## Source
Lines 324–353 in `crates/oxide-app/src/preferences/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences](/crates/oxide-app/src/preferences/mod.md) |
| calls | [build_dialog](/crates/oxide-app/src/preferences/mod/build_dialog.md) |
