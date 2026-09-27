---
okf_version: "0.2"
type: Function
title: ctx
resource: crates/oxide-erc/src/rules/ambiguous_label_anchor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/ambiguous_label_anchor/ctx
language: rust
---

# ctx

## Signature

```rust
fn ctx(wires: Vec<ErcWire>, junctions: Vec<ErcJunction>, labels: Vec<ErcLabel>) -> ErcContext
```

## Source
Lines 99–113 in `crates/oxide-erc/src/rules/ambiguous_label_anchor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ambiguous_label_anchor](/crates/oxide-erc/src/rules/ambiguous_label_anchor.md) |
| called_by | [a_junction_at_the_crossing_clears_the_ambiguity](/crates/oxide-erc/src/rules/ambiguous_label_anchor/a_junction_at_the_crossing_clears_the_ambiguity.md) |
| called_by | [a_label_on_a_shared_endpoint_is_not_flagged](/crates/oxide-erc/src/rules/ambiguous_label_anchor/a_label_on_a_shared_endpoint_is_not_flagged.md) |
| called_by | [a_label_on_a_single_wires_interior_is_not_flagged](/crates/oxide-erc/src/rules/ambiguous_label_anchor/a_label_on_a_single_wires_interior_is_not_flagged.md) |
| called_by | [a_label_on_an_undotted_crossing_is_flagged](/crates/oxide-erc/src/rules/ambiguous_label_anchor/a_label_on_an_undotted_crossing_is_flagged.md) |
