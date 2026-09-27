---
okf_version: "0.2"
type: Function
title: a_label_on_an_undotted_crossing_is_flagged
description: "The X crossing the netlist resolves by tiebreak: two wires cross at"
resource: crates/oxide-erc/src/rules/ambiguous_label_anchor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/ambiguous_label_anchor/a_label_on_an_undotted_crossing_is_flagged
language: rust
---

# a_label_on_an_undotted_crossing_is_flagged

The X crossing the netlist resolves by tiebreak: two wires cross at

## Signature

```rust
fn a_label_on_an_undotted_crossing_is_flagged()
```

## Decorators

- `test`

## Docstring

The X crossing the netlist resolves by tiebreak: two wires cross at
(5,0), no dot, a label on the crossing. The user cannot predict which
of the two nets `NET` names, so ERC has to say so.
[test]

## Source
Lines 119–131 in `crates/oxide-erc/src/rules/ambiguous_label_anchor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ambiguous_label_anchor](/crates/oxide-erc/src/rules/ambiguous_label_anchor.md) |
| calls | [ctx](/crates/oxide-erc/src/rules/ambiguous_label_anchor/ctx.md) |
| calls | [ambiguous_label_anchor](/crates/oxide-erc/src/rules/ambiguous_label_anchor/ambiguous_label_anchor.md) |
