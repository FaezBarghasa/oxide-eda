---
okf_version: "0.2"
type: Class
title: KeyBind
description: "A command's suggested default keyboard shortcut, carried on its"
resource: crates/oxide-app/src/keymap/catalog/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/catalog/mod/KeyBind
language: rust
---

# KeyBind

A command's suggested default keyboard shortcut, carried on its

## Signature

```rust
pub struct KeyBind
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

A command's suggested default keyboard shortcut, carried on its
catalog entry. Distinct from a bound [`crate::keymap::KeyStroke`] in a
keymap profile: a profile's own binding always overrides this
default. `key` is a canonical token spelling (e.g. `"c"`, `"delete"`,
`"f1"`), matching [`crate::keymap::KeyToken`]'s serde naming.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Methods

- `modifiers`
- `key`

## Source
Lines 59–62 in `crates/oxide-app/src/keymap/catalog/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [catalog](/crates/oxide-app/src/keymap/catalog/mod.md) |
