---
okf_version: "0.2"
type: Function
title: default_component_classes
description: "Materialise the seed list as owned `ComponentClassEntry` values."
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/default_component_classes
language: rust
---

# default_component_classes

Materialise the seed list as owned `ComponentClassEntry` values.

## Signature

```rust
pub fn default_component_classes() -> Vec<ComponentClassEntry>
```

## Visibility

- `pub`

## Docstring

Materialise the seed list as owned `ComponentClassEntry` values.

## Source
Lines 113–121 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| called_by | [handle_preferences_message](/crates/oxide-app/src/app/handlers/preferences/mod/handle_preferences_message.md) |
| called_by | [read_component_classes_pref](/crates/oxide-app/src/fonts/mod/read_component_classes_pref.md) |
