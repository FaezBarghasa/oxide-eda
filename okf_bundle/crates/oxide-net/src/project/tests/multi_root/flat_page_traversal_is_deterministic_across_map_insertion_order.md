---
okf_version: "0.2"
type: Function
title: flat_page_traversal_is_deterministic_across_map_insertion_order
description: "8 ── Determinism: the result must not depend on the `sheets` map's hash"
resource: crates/oxide-net/src/project/tests/multi_root.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/multi_root/flat_page_traversal_is_deterministic_across_map_insertion_order
language: rust
---

# flat_page_traversal_is_deterministic_across_map_insertion_order

8 ── Determinism: the result must not depend on the `sheets` map's hash

## Signature

```rust
fn flat_page_traversal_is_deterministic_across_map_insertion_order()
```

## Decorators

- `test`

## Docstring

8 ── Determinism: the result must not depend on the `sheets` map's hash
order. Root order is the caller's to fix — `ProjectGraph.roots` is an
ordered slice precisely so it is never left to a hash — and what this
pins is that nothing *else* in the traversal leaks map order.
[test]

## Source
Lines 337–359 in `crates/oxide-net/src/project/tests/multi_root.rs`

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
