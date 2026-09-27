---
okf_version: "0.2"
type: Function
title: a_user_placed_junction_survives_reconcile_even_when_unjustified
description: "The other half of #422: a user-placed dot is user data. It must never"
resource: crates/oxide-engine/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T12:59:13Z"
concept_id: crates/oxide-engine/src/lib/a_user_placed_junction_survives_reconcile_even_when_unjustified
language: rust
---

# a_user_placed_junction_survives_reconcile_even_when_unjustified

The other half of #422: a user-placed dot is user data. It must never

## Signature

```rust
fn a_user_placed_junction_survives_reconcile_even_when_unjustified()
```

## Decorators

- `test`

## Docstring

The other half of #422: a user-placed dot is user data. It must never
be swept up by the same removal pass just because it doesn't happen to
sit on a wire meeting.
[test]

## Source
Lines 847–889 in `crates/oxide-engine/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-engine/src/lib.md) |
| calls | [test_sheet](/crates/oxide-engine/src/test_support/test_sheet.md) |
| calls | [wire](/crates/oxide-engine/src/lib/wire.md) |
