---
okf_version: "0.2"
type: Function
title: flat_siblings_with_distinct_local_labels_stay_separate
description: "3 ── Local `Net` labels never cross sheets (rule 4) — two flat pages with"
resource: crates/oxide-net/src/project/tests/multi_root.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/multi_root/flat_siblings_with_distinct_local_labels_stay_separate
language: rust
---

# flat_siblings_with_distinct_local_labels_stay_separate

3 ── Local `Net` labels never cross sheets (rule 4) — two flat pages with

## Signature

```rust
fn flat_siblings_with_distinct_local_labels_stay_separate()
```

## Decorators

- `test`

## Docstring

3 ── Local `Net` labels never cross sheets (rule 4) — two flat pages with
distinct local names stay two distinct nets.
[test]

## Source
Lines 139–169 in `crates/oxide-net/src/project/tests/multi_root.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_root](/crates/oxide-net/src/project/tests/multi_root.md) |
| calls | [empty_sheet](/crates/oxide-net/src/project/tests/empty_sheet.md) |
| calls | [wire](/crates/oxide-net/src/project/tests/wire.md) |
| calls | [pt](/crates/oxide-net/src/project/tests/pt.md) |
| calls | [label](/crates/oxide-net/src/project/tests/label.md) |
| calls | [add_lib](/crates/oxide-net/src/project/tests/add_lib.md) |
| calls | [place](/crates/oxide-net/src/project/tests/place.md) |
| calls | [stitch_pages](/crates/oxide-net/src/project/tests/stitch_pages.md) |
| calls | [names](/crates/oxide-net/src/project/tests/names.md) |
