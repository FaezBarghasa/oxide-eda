---
okf_version: "0.2"
type: Function
title: content_component_classes
resource: crates/oxide-app/src/preferences/component_classes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/preferences/component_classes/content_component_classes
language: rust
---

# content_component_classes

## Signature

```rust
pub(super) fn content_component_classes(
    classes: &'a [crate::fonts::ComponentClassEntry],
) -> Element<'a, PrefMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Source
Lines 9–97 in `crates/oxide-app/src/preferences/component_classes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [component_classes](/crates/oxide-app/src/preferences/component_classes.md) |
| called_by | [build_content](/crates/oxide-app/src/preferences/mod/build_content.md) |
