---
okf_version: "0.2"
type: Function
title: read_component_classes_pref
description: "Read the user's component-class list from the prefs file. Falls"
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/read_component_classes_pref
language: rust
---

# read_component_classes_pref

Read the user's component-class list from the prefs file. Falls

## Signature

```rust
pub fn read_component_classes_pref() -> Vec<ComponentClassEntry>
```

## Visibility

- `pub`

## Docstring

Read the user's component-class list from the prefs file. Falls
back to [`default_component_classes`] only when the file is
absent / malformed, or when the `component_classes` key is
missing entirely (a fresh install). A user who has the array
present and empty is honored verbatim — the New Component
surface handles the "no classes defined" case at the point of
use, so saving an empty list and reading it back round-trips
faithfully.

## Source
Lines 413–427 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
| calls | [default_component_classes](/crates/oxide-app/src/fonts/mod/default_component_classes.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
| called_by | [create_library_at](/crates/oxide-app/src/library/commands/create_library_at.md) |
| called_by | [materialize_pending_library](/crates/oxide-app/src/library/commands/materialize_pending_library.md) |
