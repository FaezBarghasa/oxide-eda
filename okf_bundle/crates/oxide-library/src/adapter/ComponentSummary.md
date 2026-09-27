---
okf_version: "0.2"
type: Class
title: ComponentSummary
description: One result row from a library query — header info derived from a
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/ComponentSummary
language: rust
---

# ComponentSummary

One result row from a library query — header info derived from a

## Signature

```rust
pub struct ComponentSummary
```

## Decorators

- `derive(Clone, Debug, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

One result row from a library query — header info derived from a
[`ComponentRow`].

Used to be tied to the per-component head revision; now it's just a
thin projection of a row's display fields. Kept around for UI grids
that don't want to materialise full row payloads.
[derive(Clone, Debug, Serialize, Deserialize)]

## Methods

- `row_id`
- `internal_pn`
- `mpn`
- `state`
- `description`

## Source
Lines 65–71 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
