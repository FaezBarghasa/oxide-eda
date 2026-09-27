---
okf_version: "0.2"
type: Function
title: from_snxlib_round_trips_database_mode
description: "`from_snxlib` mirrors `LocalGitAdapter::init`'s manifest API"
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/from_snxlib_round_trips_database_mode
language: rust
---

# from_snxlib_round_trips_database_mode

`from_snxlib` mirrors `LocalGitAdapter::init`'s manifest API

## Signature

```rust
fn from_snxlib_round_trips_database_mode()
```

## Decorators

- `test`

## Docstring

`from_snxlib` mirrors `LocalGitAdapter::init`'s manifest API
shape. The DB adapter requires `LibraryMode::Database`; passing
the default `LocalGit` mode must fail loudly so a misconfigured
project doesn't silently bring up a remote-shaped adapter
pointed at nothing.
[test]

## Source
Lines 547–566 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
