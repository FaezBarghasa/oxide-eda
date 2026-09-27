---
okf_version: "0.2"
type: Function
title: sim_set_pin_node_empty_removes_key
description: "Empty value removes the key from `default_node_map`."
resource: crates/oxide-app/src/library/editor/sim/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:12:05Z"
concept_id: crates/oxide-app/src/library/editor/sim/mod/sim_set_pin_node_empty_removes_key
language: rust
---

# sim_set_pin_node_empty_removes_key

Empty value removes the key from `default_node_map`.

## Signature

```rust
fn sim_set_pin_node_empty_removes_key()
```

## Decorators

- `test`

## Docstring

Empty value removes the key from `default_node_map`.
[test]

## Source
Lines 420–443 in `crates/oxide-app/src/library/editor/sim/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-app/src/library/editor/sim/mod.md) |
| calls | [fixture_editor](/crates/oxide-app/src/library/editor/sim/mod/fixture_editor.md) |
| calls | [apply](/crates/oxide-app/src/library/editor/sim/mod/apply.md) |
