---
okf_version: "0.2"
type: Function
title: quitting_after_a_move_selection_warns_instead_of_discarding_it
description: "The user-visible failure: the edit is applied and on screen, but"
resource: crates/oxide-app/tests/regression/dirty_paths_gateway.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/dirty_paths_gateway/quitting_after_a_move_selection_warns_instead_of_discarding_it
language: rust
---

# quitting_after_a_move_selection_warns_instead_of_discarding_it

The user-visible failure: the edit is applied and on screen, but

## Signature

```rust
fn quitting_after_a_move_selection_warns_instead_of_discarding_it()
```

## Decorators

- `test`

## Docstring

The user-visible failure: the edit is applied and on screen, but
quitting does not warn, so it goes out with the process.
[test]

## Source
Lines 158–176 in `crates/oxide-app/tests/regression/dirty_paths_gateway.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dirty_paths_gateway](/crates/oxide-app/tests/regression/dirty_paths_gateway.md) |
| calls | [app_with](/crates/oxide-app/tests/regression/dirty_paths_gateway/app_with.md) |
| calls | [move_selection](/crates/oxide-app/tests/regression/dirty_paths_gateway/move_selection.md) |
