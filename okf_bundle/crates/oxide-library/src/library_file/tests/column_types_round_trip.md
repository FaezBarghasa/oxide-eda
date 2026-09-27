---
okf_version: "0.2"
type: Function
title: column_types_round_trip
description: "`column_types` map round-trips through the"
resource: crates/oxide-library/src/library_file/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/tests/column_types_round_trip
language: rust
---

# column_types_round_trip

`column_types` map round-trips through the

## Signature

```rust
fn column_types_round_trip()
```

## Decorators

- `test`

## Docstring

`column_types` map round-trips through the
`[tables.<name>.column_types]` sidecar — the type tokens
(`number`, `bool`, `enum:active,preferred,...`) survive parse +
write unchanged.
[test]

## Source
Lines 500–542 in `crates/oxide-library/src/library_file/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/library_file/tests.md) |
| calls | [fixture_manifest](/crates/oxide-library/src/library_file/tests/fixture_manifest.md) |
