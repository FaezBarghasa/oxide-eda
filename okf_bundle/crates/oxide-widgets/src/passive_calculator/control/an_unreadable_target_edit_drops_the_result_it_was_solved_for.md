---
okf_version: "0.2"
type: Function
title: an_unreadable_target_edit_drops_the_result_it_was_solved_for
description: "GH #599 — the result used to survive an edit that left the target"
resource: crates/oxide-widgets/src/passive_calculator/control.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/passive_calculator/control/an_unreadable_target_edit_drops_the_result_it_was_solved_for
language: rust
---

# an_unreadable_target_edit_drops_the_result_it_was_solved_for

GH #599 — the result used to survive an edit that left the target

## Signature

```rust
fn an_unreadable_target_edit_drops_the_result_it_was_solved_for()
```

## Decorators

- `test`

## Docstring

GH #599 — the result used to survive an edit that left the target
unreadable, and the summary then rendered against `target = 0.0`:
"Target" read 0 and all three deltas were measured against 0 while
Nominal / Minimum / Maximum still showed the real network. No
error text either, because TargetChanged clears validation_error.
[test]

## Source
Lines 544–551 in `crates/oxide-widgets/src/passive_calculator/control.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [control](/crates/oxide-widgets/src/passive_calculator/control.md) |
| calls | [calculated](/crates/oxide-widgets/src/passive_calculator/control/calculated.md) |
