---
okf_version: "0.2"
type: Function
title: view_body
description: Render the Preferences dialog body without the centred backdrop —
resource: crates/oxide-app/src/preferences/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/preferences/mod/view_body
language: rust
---

# view_body

Render the Preferences dialog body without the centred backdrop —

## Signature

```rust
pub(crate) fn view_body(v: PrefsView<'a>) -> Element<'a, PrefMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(crate)`

## Docstring

Render the Preferences dialog body without the centred backdrop —
used by `view_detached_modal` so the dialog fills its own OS window.
In-window callers go through `view()` which wraps this in a tinted
dismiss layer.

## Source
Lines 361–363 in `crates/oxide-app/src/preferences/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences](/crates/oxide-app/src/preferences/mod.md) |
| calls | [build_dialog](/crates/oxide-app/src/preferences/mod/build_dialog.md) |
| called_by | [view_preferences_body](/crates/oxide-app/src/app/view/chrome/view_preferences_body.md) |
