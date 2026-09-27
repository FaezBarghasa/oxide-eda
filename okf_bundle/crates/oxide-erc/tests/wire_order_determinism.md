---
okf_version: "0.2"
type: Module
title: wire_order_determinism
description: "ERC's verdict must not depend on the order wires sit in the document."
resource: crates/oxide-erc/tests/wire_order_determinism.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/tests/wire_order_determinism
language: rust
---

# wire_order_determinism

ERC's verdict must not depend on the order wires sit in the document.

## Docstring

ERC's verdict must not depend on the order wires sit in the document.

Every net rule reads `SheetConnectivity::root_of_anchored`, which used to
anchor a point into the *first* wire whose segment it touched. At a
junction-less T — a point that is one wire's own endpoint and another wire's
interior — that either bridged the two wires into one net or left them
separate, decided purely by which wire the slice yielded first. Two
conflicting labels straddling the T therefore reported a conflict or not
depending on draw order (issue #402).

Asserting the whole violation multiset (not just one rule) covers every rule
that routes through the shared anchor, `derive_nets` included.

## Relationships

| Type | Target |
|------|--------|
| related | [pt](/crates/oxide-erc/tests/wire_order_determinism/pt.md) |
| related | [sheet](/crates/oxide-erc/tests/wire_order_determinism/sheet.md) |
| related | [wire](/crates/oxide-erc/tests/wire_order_determinism/wire.md) |
| related | [net_label](/crates/oxide-erc/tests/wire_order_determinism/net_label.md) |
| related | [fingerprint](/crates/oxide-erc/tests/wire_order_determinism/fingerprint.md) |
| related | [conflicting_t](/crates/oxide-erc/tests/wire_order_determinism/conflicting_t.md) |
| related | [erc_verdict_is_independent_of_wire_order_at_a_junction_less_t](/crates/oxide-erc/tests/wire_order_determinism/erc_verdict_is_independent_of_wire_order_at_a_junction_less_t.md) |
| related | [net_label_conflict_count_is_independent_of_wire_order](/crates/oxide-erc/tests/wire_order_determinism/net_label_conflict_count_is_independent_of_wire_order.md) |
