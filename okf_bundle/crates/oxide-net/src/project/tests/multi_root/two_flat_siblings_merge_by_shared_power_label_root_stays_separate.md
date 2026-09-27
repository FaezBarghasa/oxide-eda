---
okf_version: "0.2"
type: Function
title: two_flat_siblings_merge_by_shared_power_label_root_stays_separate
description: 2 ── Two flat siblings (neither referenced by root nor by each other) merge
resource: crates/oxide-net/src/project/tests/multi_root.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/multi_root/two_flat_siblings_merge_by_shared_power_label_root_stays_separate
language: rust
---

# two_flat_siblings_merge_by_shared_power_label_root_stays_separate

2 ── Two flat siblings (neither referenced by root nor by each other) merge

## Signature

```rust
fn two_flat_siblings_merge_by_shared_power_label_root_stays_separate()
```

## Decorators

- `test`

## Docstring

2 ── Two flat siblings (neither referenced by root nor by each other) merge
with each other by a shared Power label, while the root's own,
unrelated net stays untouched.
[test]

## Source
Lines 81–134 in `crates/oxide-net/src/project/tests/multi_root.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_root](/crates/oxide-net/src/project/tests/multi_root.md) |
| calls | [empty_sheet](/crates/oxide-net/src/project/tests/empty_sheet.md) |
| calls | [wire](/crates/oxide-net/src/project/tests/wire.md) |
| calls | [pt](/crates/oxide-net/src/project/tests/pt.md) |
| calls | [add_lib](/crates/oxide-net/src/project/tests/add_lib.md) |
| calls | [place](/crates/oxide-net/src/project/tests/place.md) |
| calls | [place_power](/crates/oxide-net/src/project/tests/place_power.md) |
| calls | [label](/crates/oxide-net/src/project/tests/label.md) |
| calls | [stitch_pages](/crates/oxide-net/src/project/tests/stitch_pages.md) |
| calls | [find](/crates/oxide-net/src/uf/find.md) |
