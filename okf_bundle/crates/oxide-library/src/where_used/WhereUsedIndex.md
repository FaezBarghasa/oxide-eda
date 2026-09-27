---
okf_version: "0.2"
type: Class
title: WhereUsedIndex
description: "Reverse index from row id → list of [`UseSite`]."
resource: crates/oxide-library/src/where_used.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/where_used/WhereUsedIndex
language: rust
---

# WhereUsedIndex

Reverse index from row id → list of [`UseSite`].

## Signature

```rust
pub struct WhereUsedIndex
```

## Decorators

- `derive(Default)`

## Visibility

- `pub`

## Docstring

Reverse index from row id → list of [`UseSite`].

Built incrementally — call [`ingest_sheet`](Self::ingest_sheet) when a sheet
is opened or saved, and [`drop_project`](Self::drop_project) when a project
is closed. Look up via [`where_used`](Self::where_used).

L3: SAFETY — this index is **not** `Sync`. The mutating API (`ingest_sheet`,
`drop_project`) takes `&mut self` and the read API (`where_used`) takes
`&self`, but the consumer is the UI thread which serialises every call
through the iced update loop. There is no need for interior mutability and
no plan to share the index across worker threads. The `PhantomData<Cell<()>>`
marker opts out of `Sync` so a future refactor can't accidentally hand the
index to a background thread without the compiler shouting first.

Stays `Send` so single-thread ownership transfer (e.g. moving into a
`tokio::task::spawn_blocking` closure) keeps working.
[derive(Default)]

## Methods

- `by_project`
- `primitive_to_rows`
- `_not_sync`

## Source
Lines 68–79 in `crates/oxide-library/src/where_used.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [where_used](/crates/oxide-library/src/where_used.md) |
