---
okf_version: "0.2"
type: Function
title: sim_set_enabled_true_is_idempotent_when_already_bound
description: Enable on an editor that already has a sim is idempotent
resource: crates/oxide-app/src/library/editor/sim/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:12:05Z"
concept_id: crates/oxide-app/src/library/editor/sim/mod/sim_set_enabled_true_is_idempotent_when_already_bound
language: rust
---

# sim_set_enabled_true_is_idempotent_when_already_bound

Enable on an editor that already has a sim is idempotent

## Signature

```rust
fn sim_set_enabled_true_is_idempotent_when_already_bound()
```

## Decorators

- `test`

## Docstring

Enable on an editor that already has a sim is idempotent
(no clobber).
[test]

## Source
Lines 486–493 in `crates/oxide-app/src/library/editor/sim/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-app/src/library/editor/sim/mod.md) |
| calls | [fixture_editor](/crates/oxide-app/src/library/editor/sim/mod/fixture_editor.md) |
| calls | [apply](/crates/oxide-app/src/library/editor/sim/mod/apply.md) |
