---
okf_version: "0.2"
type: Class
title: InputTarget
description: "Where an input event landed, and therefore what the user can see"
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/InputTarget
language: rust
---

# InputTarget

Where an input event landed, and therefore what the user can see

## Signature

```rust
pub(crate) enum InputTarget
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub(crate)`

## Docstring

Where an input event landed, and therefore what the user can see
while pressing the key.

This is #554's `EscapeSource` promoted: Esc was merely the first key
that needed to know its window. Derived from `WindowKind` by an
exhaustive match, so a new window kind is a compile error rather than
a silent inheritance of the main window's behaviour.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 81–97 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
