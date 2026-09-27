---
okf_version: "0.2"
type: Function
title: without_a_root_the_winner_is_still_sorted_first_wins
description: "`None` is what ERC passes, and it must keep the old rule exactly:"
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/without_a_root_the_winner_is_still_sorted_first_wins
language: rust
---

# without_a_root_the_winner_is_still_sorted_first_wins

`None` is what ERC passes, and it must keep the old rule exactly:

## Signature

```rust
fn without_a_root_the_winner_is_still_sorted_first_wins()
```

## Decorators

- `test`

## Docstring

`None` is what ERC passes, and it must keep the old rule exactly:
sorted-path first-wins, no notion of a root. Without this the
exemption could quietly become "whichever path the caller happened
to mention", which is not a total order.
[test]

## Source
Lines 1046–1058 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
| calls | [sheet](/crates/oxide-app/src/app/project_sheets/sheet.md) |
| calls | [project_graph](/crates/oxide-app/src/app/project_sheets/project_graph.md) |
