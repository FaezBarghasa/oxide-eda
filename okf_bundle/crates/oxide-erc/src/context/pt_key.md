---
okf_version: "0.2"
type: Function
title: pt_key
description: "MD-6: see `rules::key` — the 1 µm bucket, the single \"same point\" metric so"
resource: crates/oxide-erc/src/context.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/context/pt_key
language: rust
---

# pt_key

MD-6: see `rules::key` — the 1 µm bucket, the single "same point" metric so

## Signature

```rust
fn pt_key(p: &Point) -> (i64, i64)
```

## Docstring

MD-6: see `rules::key` — the 1 µm bucket, the single "same point" metric so
context + rule projections agree on net membership (D5.5).

## Source
Lines 369–371 in `crates/oxide-erc/src/context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context](/crates/oxide-erc/src/context.md) |
| called_by | [junction_is_honoured](/crates/oxide-engine/src/transform/autoplace/junction_is_honoured.md) |
| called_by | [point_is_connected](/crates/oxide-erc/src/context/point_is_connected.md) |
| called_by | [ambiguous_label_anchor](/crates/oxide-erc/src/rules/ambiguous_label_anchor/ambiguous_label_anchor.md) |
| called_by | [on_any_bus_endpoint](/crates/oxide-erc/src/rules/mod/on_any_bus_endpoint.md) |
| called_by | [on_any_wire](/crates/oxide-erc/src/rules/mod/on_any_wire.md) |
| called_by | [orphan_label](/crates/oxide-erc/src/rules/mod/orphan_label.md) |
| called_by | [analyze](/crates/oxide-net/src/project/mod/analyze.md) |
