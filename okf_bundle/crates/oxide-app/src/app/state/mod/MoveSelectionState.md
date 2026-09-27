---
okf_version: "0.2"
type: Class
title: MoveSelectionState
description: Transient state for the Altium-style Move Selection dialog.
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/MoveSelectionState
language: rust
---

# MoveSelectionState

Transient state for the Altium-style Move Selection dialog.

## Signature

```rust
pub struct MoveSelectionState
```

## Decorators

- `derive(Debug, Clone, Default)`

## Visibility

- `pub`

## Docstring

Transient state for the Altium-style Move Selection dialog.
Deltas are stored as strings so mid-edit partial values (`-`, `2.`)
don't panic through number parsing; the Apply handler parses them.
[derive(Debug, Clone, Default)]

## Methods

- `open`
- `dx`
- `dy`

## Source
Lines 137–141 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
