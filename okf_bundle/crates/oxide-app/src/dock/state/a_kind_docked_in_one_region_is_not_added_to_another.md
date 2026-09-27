---
okf_version: "0.2"
type: Function
title: a_kind_docked_in_one_region_is_not_added_to_another
description: "#641: the guard used to be per-region, so the same kind could"
resource: crates/oxide-app/src/dock/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/dock/state/a_kind_docked_in_one_region_is_not_added_to_another
language: rust
---

# a_kind_docked_in_one_region_is_not_added_to_another

#641: the guard used to be per-region, so the same kind could

## Signature

```rust
fn a_kind_docked_in_one_region_is_not_added_to_another()
```

## Decorators

- `test`

## Docstring

#641: the guard used to be per-region, so the same kind could
be docked in all three at once.
[test]

## Source
Lines 356–366 in `crates/oxide-app/src/dock/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/dock/state.md) |
