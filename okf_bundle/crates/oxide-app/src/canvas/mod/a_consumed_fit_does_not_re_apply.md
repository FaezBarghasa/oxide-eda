---
okf_version: "0.2"
type: Function
title: a_consumed_fit_does_not_re_apply
description: "A second consecutive call has nothing to consume. Before #632 the"
resource: crates/oxide-app/src/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/canvas/mod/a_consumed_fit_does_not_re_apply
language: rust
---

# a_consumed_fit_does_not_re_apply

A second consecutive call has nothing to consume. Before #632 the

## Signature

```rust
fn a_consumed_fit_does_not_re_apply()
```

## Decorators

- `test`

## Docstring

A second consecutive call has nothing to consume. Before #632 the
second `take()` (from `CanvasState`) made this ambiguous; now the
absence of a pending request must simply be a no-op, so a fit does not
keep re-applying and fighting the user's pan on every later event.
[test]

## Source
Lines 846–870 in `crates/oxide-app/src/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/canvas/mod.md) |
| calls | [test_prefs](/crates/oxide-app/src/canvas/mod/test_prefs.md) |
