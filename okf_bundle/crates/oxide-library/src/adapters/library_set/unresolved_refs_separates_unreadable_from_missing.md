---
okf_version: "0.2"
type: Function
title: unresolved_refs_separates_unreadable_from_missing
description: "The sweep must not file an unreadable library under \"missing\" —"
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/unresolved_refs_separates_unreadable_from_missing
language: rust
---

# unresolved_refs_separates_unreadable_from_missing

The sweep must not file an unreadable library under "missing" —

## Signature

```rust
fn unresolved_refs_separates_unreadable_from_missing()
```

## Decorators

- `test`

## Docstring

The sweep must not file an unreadable library under "missing" —
that is the register's wrong-binding report all over again, just
in bulk.
[test]

## Source
Lines 557–587 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
