---
okf_version: "0.2"
type: Function
title: tables_overrides_round_trip_and_resolve
description: "Step 1.5 from the plan: tables overrides round-trip through TOML and"
resource: crates/oxide-library/src/manifest.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/manifest/tables_overrides_round_trip_and_resolve
language: rust
---

# tables_overrides_round_trip_and_resolve

Step 1.5 from the plan: tables overrides round-trip through TOML and

## Signature

```rust
fn tables_overrides_round_trip_and_resolve()
```

## Decorators

- `test`

## Docstring

Step 1.5 from the plan: tables overrides round-trip through TOML and
`table_for_class` honours the explicit `classes` list.
[test]

## Source
Lines 279–305 in `crates/oxide-library/src/manifest.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manifest](/crates/oxide-library/src/manifest.md) |
