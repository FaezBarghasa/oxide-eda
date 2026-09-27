---
okf_version: "0.2"
type: Function
title: in_place_remint_records_the_ledger_against_the_real_sketch
description: "#433 review — the id-preserving IN-PLACE re-mint copied the centre's"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/in_place_remint_records_the_ledger_against_the_real_sketch
language: rust
---

# in_place_remint_records_the_ledger_against_the_real_sketch

#433 review — the id-preserving IN-PLACE re-mint copied the centre's

## Signature

```rust
fn in_place_remint_records_the_ledger_against_the_real_sketch()
```

## Decorators

- `test`

## Docstring

#433 review — the id-preserving IN-PLACE re-mint copied the centre's
`PadAttr` from a SCRATCH reference mint, so the durable `owned` ledger
ended up naming scratch-only ids that never existed in the real sketch.
After save+reopen (which resets the volatile `corner_entity_ids` /
`shape_params` fields) that ledger is the SOLE owner set, so the
stranded ids would strand the outline on the next move / rotate —
wrong copper, no warning. The in-place path now re-records the ledger
against the real sketch; every owned id must be a live entity.
[test]

## Source
Lines 918–964 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| calls | [editor_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/editor_pad.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
