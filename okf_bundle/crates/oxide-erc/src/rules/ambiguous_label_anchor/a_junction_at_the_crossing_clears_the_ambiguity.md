---
okf_version: "0.2"
type: Function
title: a_junction_at_the_crossing_clears_the_ambiguity
description: "A dot at the crossing makes the two wires one net, so whichever segment"
resource: crates/oxide-erc/src/rules/ambiguous_label_anchor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/ambiguous_label_anchor/a_junction_at_the_crossing_clears_the_ambiguity
language: rust
---

# a_junction_at_the_crossing_clears_the_ambiguity

A dot at the crossing makes the two wires one net, so whichever segment

## Signature

```rust
fn a_junction_at_the_crossing_clears_the_ambiguity()
```

## Decorators

- `test`

## Docstring

A dot at the crossing makes the two wires one net, so whichever segment
the tiebreak picks names the same thing — nothing ambiguous left.
[test]

## Source
Lines 136–150 in `crates/oxide-erc/src/rules/ambiguous_label_anchor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ambiguous_label_anchor](/crates/oxide-erc/src/rules/ambiguous_label_anchor.md) |
| calls | [ctx](/crates/oxide-erc/src/rules/ambiguous_label_anchor/ctx.md) |
| calls | [ambiguous_label_anchor](/crates/oxide-erc/src/rules/ambiguous_label_anchor/ambiguous_label_anchor.md) |
