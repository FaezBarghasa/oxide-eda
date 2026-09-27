---
okf_version: "0.2"
type: Class
title: ActiveBarAction
description: All actions available from Active Bar buttons and dropdown items.
resource: crates/oxide-app/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/active_bar/mod/ActiveBarAction
language: rust
---

# ActiveBarAction

All actions available from Active Bar buttons and dropdown items.

## Signature

```rust
pub enum ActiveBarAction
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

All actions available from Active Bar buttons and dropdown items.

`PartialEq` so `app::command::active_bar` can look an action up in its
id table by value, rather than formatting `Debug` on every dropdown
row render (#271).
[derive(Debug, Clone, PartialEq)]

## Source
Lines 268–367 in `crates/oxide-app/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/active_bar/mod.md) |
