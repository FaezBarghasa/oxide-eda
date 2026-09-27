---
okf_version: "0.2"
type: Function
title: two_flat_pages_referencing_each_other_is_a_cycle_not_a_hang
description: 7 ── Two flat pages that reference each other (neither reachable from root)
resource: crates/oxide-net/src/project/tests/multi_root.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/multi_root/two_flat_pages_referencing_each_other_is_a_cycle_not_a_hang
language: rust
---

# two_flat_pages_referencing_each_other_is_a_cycle_not_a_hang

7 ── Two flat pages that reference each other (neither reachable from root)

## Signature

```rust
fn two_flat_pages_referencing_each_other_is_a_cycle_not_a_hang()
```

## Decorators

- `test`

## Docstring

7 ── Two flat pages that reference each other (neither reachable from root)
close a cycle instead of hanging the traversal.
[test]

## Source
Lines 307–330 in `crates/oxide-net/src/project/tests/multi_root.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_root](/crates/oxide-net/src/project/tests/multi_root.md) |
| calls | [empty_sheet](/crates/oxide-net/src/project/tests/empty_sheet.md) |
| calls | [child_sheet](/crates/oxide-net/src/project/tests/child_sheet.md) |
| calls | [stitch_pages](/crates/oxide-net/src/project/tests/stitch_pages.md) |
