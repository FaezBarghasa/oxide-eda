---
okf_version: "0.2"
type: Function
title: the_table_has_no_duplicates
description: "Ids must be unique, and no action may be listed twice."
resource: crates/oxide-app/src/app/command/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/command/active_bar/the_table_has_no_duplicates
language: rust
---

# the_table_has_no_duplicates

Ids must be unique, and no action may be listed twice.

## Signature

```rust
fn the_table_has_no_duplicates()
```

## Decorators

- `test`

## Docstring

Ids must be unique, and no action may be listed twice.
[test]

## Source
Lines 277–293 in `crates/oxide-app/src/app/command/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/app/command/active_bar.md) |
| calls | [command_ids](/crates/oxide-app/src/app/command/active_bar/command_ids.md) |
