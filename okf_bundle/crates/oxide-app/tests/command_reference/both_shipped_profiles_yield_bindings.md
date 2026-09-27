---
okf_version: "0.2"
type: Function
title: both_shipped_profiles_yield_bindings
description: The binding parser must actually find bindings. A silent zero would
resource: crates/oxide-app/tests/command_reference.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/command_reference/both_shipped_profiles_yield_bindings
language: rust
---

# both_shipped_profiles_yield_bindings

The binding parser must actually find bindings. A silent zero would

## Signature

```rust
fn both_shipped_profiles_yield_bindings()
```

## Decorators

- `test`

## Docstring

The binding parser must actually find bindings. A silent zero would
render every shortcut column as `—` and still match a golden that had
been regenerated from the same broken parse.
[test]

## Source
Lines 157–174 in `crates/oxide-app/tests/command_reference.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command_reference](/crates/oxide-app/tests/command_reference.md) |
| calls | [bindings](/crates/oxide-app/tests/command_reference/bindings.md) |
