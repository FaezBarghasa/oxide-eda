---
okf_version: "0.2"
type: Function
title: a_label_on_a_shared_endpoint_is_not_flagged
description: "A label at the shared endpoint of two wires: `anchor_point` rule 1"
resource: crates/oxide-erc/src/rules/ambiguous_label_anchor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/ambiguous_label_anchor/a_label_on_a_shared_endpoint_is_not_flagged
language: rust
---

# a_label_on_a_shared_endpoint_is_not_flagged

A label at the shared endpoint of two wires: `anchor_point` rule 1

## Signature

```rust
fn a_label_on_a_shared_endpoint_is_not_flagged()
```

## Decorators

- `test`

## Docstring

A label at the shared endpoint of two wires: `anchor_point` rule 1
short-circuits, the label is already in that class, nothing is chosen.
[test]

## Source
Lines 168–180 in `crates/oxide-erc/src/rules/ambiguous_label_anchor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ambiguous_label_anchor](/crates/oxide-erc/src/rules/ambiguous_label_anchor.md) |
| calls | [ctx](/crates/oxide-erc/src/rules/ambiguous_label_anchor/ctx.md) |
| calls | [ambiguous_label_anchor](/crates/oxide-erc/src/rules/ambiguous_label_anchor/ambiguous_label_anchor.md) |
