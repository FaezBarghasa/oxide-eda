---
okf_version: "0.2"
type: Function
title: a_blocking_modal_always_claims_escape_itself
description: The hand-written ladder closed its blocking-modal guard with a
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/a_blocking_modal_always_claims_escape_itself
language: rust
---

# a_blocking_modal_always_claims_escape_itself

The hand-written ladder closed its blocking-modal guard with a

## Signature

```rust
fn a_blocking_modal_always_claims_escape_itself()
```

## Decorators

- `test`

## Docstring

The hand-written ladder closed its blocking-modal guard with a
`debug_assert!` that at least one of the four members really was
set once the guard had been entered. Deriving the walk from
`PAINT_ORDER` deletes the guard, so the invariant it protected
gets a test instead: in the blocking regime Esc must always be
claimed by one of the visible five and never fall through to the
tool reset.
[test]

## Source
Lines 706–749 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
