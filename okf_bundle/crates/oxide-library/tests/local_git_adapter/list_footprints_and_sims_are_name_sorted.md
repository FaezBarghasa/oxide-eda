---
okf_version: "0.2"
type: Function
title: list_footprints_and_sims_are_name_sorted
description: "`list_footprints` / `list_sims` read the name out of every"
resource: crates/oxide-library/tests/local_git_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T13:10:08Z"
concept_id: crates/oxide-library/tests/local_git_adapter/list_footprints_and_sims_are_name_sorted
language: rust
---

# list_footprints_and_sims_are_name_sorted

`list_footprints` / `list_sims` read the name out of every

## Signature

```rust
fn list_footprints_and_sims_are_name_sorted()
```

## Decorators

- `test`

## Docstring

`list_footprints` / `list_sims` read the name out of every
`<uuid>.<ext>` envelope in the per-kind directory, tag each summary
with the right [`PrimitiveKind`], and hand the whole list back
sorted by name — not by uuid and not in directory-walk order.

The save order below is deliberately not alphabetic, and the uuids
are `now_v7` so filename order tracks save order: a listing that
forgot to sort would come back in save order and fail here.
[test]

## Source
Lines 421–461 in `crates/oxide-library/tests/local_git_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_git_adapter](/crates/oxide-library/tests/local_git_adapter.md) |
| calls | [init_adapter](/crates/oxide-library/tests/local_git_adapter/init_adapter.md) |
| calls | [fixture_footprint](/crates/oxide-library/tests/local_git_adapter/fixture_footprint.md) |
| calls | [fixture_sim](/crates/oxide-library/tests/local_git_adapter/fixture_sim.md) |
