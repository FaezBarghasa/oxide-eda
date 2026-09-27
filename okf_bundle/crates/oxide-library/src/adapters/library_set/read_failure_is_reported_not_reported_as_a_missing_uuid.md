---
okf_version: "0.2"
type: Function
title: read_failure_is_reported_not_reported_as_a_missing_uuid
description: A read failure is not evidence that the binding is wrong. The
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/read_failure_is_reported_not_reported_as_a_missing_uuid
language: rust
---

# read_failure_is_reported_not_reported_as_a_missing_uuid

A read failure is not evidence that the binding is wrong. The

## Signature

```rust
fn read_failure_is_reported_not_reported_as_a_missing_uuid()
```

## Decorators

- `test`

## Docstring

A read failure is not evidence that the binding is wrong. The
resolvers must hand the caller the error instead of the same
`None` a genuinely absent UUID produces.
[test]

## Source
Lines 506–526 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
| calls | [fixture_symbol](/crates/oxide-library/src/adapters/library_set/fixture_symbol.md) |
