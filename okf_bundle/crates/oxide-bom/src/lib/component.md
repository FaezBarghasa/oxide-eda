---
okf_version: "0.2"
type: Function
title: component
resource: crates/oxide-bom/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bom"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-bom/src/lib/component
language: rust
---

# component

## Signature

```rust
fn component(reference: &str, value: &str, footprint: &str) -> BomComponent
```

## Source
Lines 376–390 in `crates/oxide-bom/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-bom/src/lib.md) |
| called_by | [can_include_excluded_from_bom_and_not_on_board](/crates/oxide-bom/src/lib/can_include_excluded_from_bom_and_not_on_board.md) |
| called_by | [filters_dnp_by_default](/crates/oxide-bom/src/lib/filters_dnp_by_default.md) |
