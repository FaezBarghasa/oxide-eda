---
okf_version: "0.2"
type: Function
title: conflicting_t
description: "A = (0,0)–(10,0), B = (5,0)–(5,10), no junction dot. B *ends* on A's"
resource: crates/oxide-erc/tests/wire_order_determinism.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/tests/wire_order_determinism/conflicting_t
language: rust
---

# conflicting_t

A = (0,0)–(10,0), B = (5,0)–(5,10), no junction dot. B *ends* on A's

## Signature

```rust
fn conflicting_t(reversed: bool) -> SchematicSheet
```

## Docstring

A = (0,0)–(10,0), B = (5,0)–(5,10), no junction dot. B *ends* on A's
interior, so (5,0) is simultaneously B's own endpoint and a point of A.

`ALPHA` sits exactly on that T point — the ambiguous one. `BETA` sits on A's
**pure** interior, away from every endpoint, so it always belongs to A. If
anchoring bridges A and B, both names land on one net and ERC reports a
`NetLabelConflict`; if it doesn't, they stay on separate nets and it
reports nothing. The old first-match anchor picked whichever wire came
first, so this one sheet produced both verdicts.

## Source
Lines 94–109 in `crates/oxide-erc/tests/wire_order_determinism.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire_order_determinism](/crates/oxide-erc/tests/wire_order_determinism.md) |
| calls | [sheet](/crates/oxide-erc/tests/wire_order_determinism/sheet.md) |
| called_by | [erc_verdict_is_independent_of_wire_order_at_a_junction_less_t](/crates/oxide-erc/tests/wire_order_determinism/erc_verdict_is_independent_of_wire_order_at_a_junction_less_t.md) |
