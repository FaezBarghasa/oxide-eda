---
okf_version: "0.2"
type: Function
title: snxsch_without_junction_extras_defaults_to_user_placed
description: "A `.snxsch` written before issue #422 added `Junction::minted` never"
resource: crates/oxide-types/src/format/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:16:55Z"
concept_id: crates/oxide-types/src/format/tests/snxsch_without_junction_extras_defaults_to_user_placed
language: rust
---

# snxsch_without_junction_extras_defaults_to_user_placed

A `.snxsch` written before issue #422 added `Junction::minted` never

## Signature

```rust
fn snxsch_without_junction_extras_defaults_to_user_placed()
```

## Decorators

- `test`

## Docstring

A `.snxsch` written before issue #422 added `Junction::minted` never
emitted `[extras.junctions.<uuid>]`. Loading one must not error and
must treat every junction it names as user-placed (never auto-removed),
not as an unminted/undefined provenance.
[test]

## Source
Lines 370–400 in `crates/oxide-types/src/format/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-types/src/format/tests.md) |
| calls | [empty_sheet](/crates/oxide-types/src/format/tests/empty_sheet.md) |
