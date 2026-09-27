---
okf_version: "0.2"
type: Class
title: Enablement
description: Fixed predicate gating when a command is enabled. Evaluating this
resource: crates/oxide-app/src/keymap/catalog/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/catalog/mod/Enablement
language: rust
---

# Enablement

Fixed predicate gating when a command is enabled. Evaluating this

## Signature

```rust
pub enum Enablement
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Default)`

## Visibility

- `pub`

## Docstring

Fixed predicate gating when a command is enabled. Evaluating this
against live application state is future work (a command
registry/dispatch consumer, tracked separately) — today it only
travels with the catalog entry.
[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]

## Source
Lines 80–90 in `crates/oxide-app/src/keymap/catalog/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [catalog](/crates/oxide-app/src/keymap/catalog/mod.md) |
