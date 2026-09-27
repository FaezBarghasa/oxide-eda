---
okf_version: "0.2"
type: Function
title: dragging_a_minted_stub_off_the_trunk_removes_the_stale_dot
description: "Issue #422: the old add-only reconcile left a minted dot in place once"
resource: crates/oxide-engine/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T12:59:13Z"
concept_id: crates/oxide-engine/src/lib/dragging_a_minted_stub_off_the_trunk_removes_the_stale_dot
language: rust
---

# dragging_a_minted_stub_off_the_trunk_removes_the_stale_dot

Issue #422: the old add-only reconcile left a minted dot in place once

## Signature

```rust
fn dragging_a_minted_stub_off_the_trunk_removes_the_stale_dot()
```

## Decorators

- `test`

## Docstring

Issue #422: the old add-only reconcile left a minted dot in place once
the T that justified it lost its stub — the dot then sat electrically
inert on the trunk alone until some unrelated wire was later dragged
to merely cross the same point, at which point the stale dot silently
merged two nets the user never connected. Dragging the stub away must
remove the dot immediately, not wait for a crossing wire to expose it.
[test]

## Source
Lines 789–841 in `crates/oxide-engine/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-engine/src/lib.md) |
| calls | [test_sheet](/crates/oxide-engine/src/test_support/test_sheet.md) |
| calls | [wire](/crates/oxide-engine/src/lib/wire.md) |
