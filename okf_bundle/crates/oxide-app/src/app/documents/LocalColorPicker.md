---
okf_version: "0.2"
type: Class
title: LocalColorPicker
description: Transient open-state for a symbol-level local-colour picker.
resource: crates/oxide-app/src/app/documents.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/documents/LocalColorPicker
language: rust
---

# LocalColorPicker

Transient open-state for a symbol-level local-colour picker.

## Signature

```rust
pub struct LocalColorPicker
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Transient open-state for a symbol-level local-colour picker.
`slot` selects Fills / Lines / Pins; `advanced` is `true` once the
user expanded into the HSV / RGB overlay. UI-only — never
serialized, never snapshotted for undo.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Methods

- `slot`
- `advanced`

## Source
Lines 225–228 in `crates/oxide-app/src/app/documents.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [documents](/crates/oxide-app/src/app/documents.md) |
