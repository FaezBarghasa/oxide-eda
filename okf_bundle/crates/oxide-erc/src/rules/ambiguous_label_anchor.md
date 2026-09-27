---
okf_version: "0.2"
type: Module
title: ambiguous_label_anchor
description: "Rule: AmbiguousLabelAnchor."
resource: crates/oxide-erc/src/rules/ambiguous_label_anchor.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/ambiguous_label_anchor
language: rust
---

# ambiguous_label_anchor

Rule: AmbiguousLabelAnchor.

## Docstring

Rule: AmbiguousLabelAnchor.

`build_netlist`'s `anchor_point` rule 2 resolves a label that sits on the
interior of *several* wires by picking exactly one — the segment with the
smallest normalised endpoint-key pair. That tiebreak is deterministic (it
has to be: issue #402 made the whole partition independent of document
order), but it is still an arbitrary **electrical** decision. Two wires
merely crossing at a point are two separate nets; the label names one of
them, and nothing on screen tells the user which.

Determinism without disclosure is the worse half of the fix, so the netlist
keeps the tiebreak and ERC says out loud that a tiebreak happened.

Not flagged:
- a label on a wire **endpoint** — `anchor_point` rule 1 returns early, the
label is already a node of that wire's class, no choice is made;
- a label where a junction sits — the crossing wires are one net there, so
whichever segment wins names the same net.

## Relationships

| Type | Target |
|------|--------|
| related | [ambiguous_label_anchor](/crates/oxide-erc/src/rules/ambiguous_label_anchor/ambiguous_label_anchor.md) |
| related | [pt](/crates/oxide-erc/src/rules/ambiguous_label_anchor/pt.md) |
| related | [wire](/crates/oxide-erc/src/rules/ambiguous_label_anchor/wire.md) |
| related | [net_label](/crates/oxide-erc/src/rules/ambiguous_label_anchor/net_label.md) |
| related | [ctx](/crates/oxide-erc/src/rules/ambiguous_label_anchor/ctx.md) |
| related | [a_label_on_an_undotted_crossing_is_flagged](/crates/oxide-erc/src/rules/ambiguous_label_anchor/a_label_on_an_undotted_crossing_is_flagged.md) |
| related | [a_junction_at_the_crossing_clears_the_ambiguity](/crates/oxide-erc/src/rules/ambiguous_label_anchor/a_junction_at_the_crossing_clears_the_ambiguity.md) |
| related | [a_label_on_a_single_wires_interior_is_not_flagged](/crates/oxide-erc/src/rules/ambiguous_label_anchor/a_label_on_a_single_wires_interior_is_not_flagged.md) |
| related | [a_label_on_a_shared_endpoint_is_not_flagged](/crates/oxide-erc/src/rules/ambiguous_label_anchor/a_label_on_a_shared_endpoint_is_not_flagged.md) |
