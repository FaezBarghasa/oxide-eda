---
okf_version: "0.2"
type: Function
title: closed_profile_seed_attrs_stay_on_line_a_only
description: "BLOCKER 1 repro. Before the fix, `build_split_entities` cloned the"
resource: crates/oxide-sketch/src/split/tests/carry_over.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/tests/carry_over/closed_profile_seed_attrs_stay_on_line_a_only
language: rust
---

# closed_profile_seed_attrs_stay_on_line_a_only

BLOCKER 1 repro. Before the fix, `build_split_entities` cloned the

## Signature

```rust
fn closed_profile_seed_attrs_stay_on_line_a_only()
```

## Decorators

- `test`

## Docstring

BLOCKER 1 repro. Before the fix, `build_split_entities` cloned the
retired Line's ENTIRE attribute set onto BOTH halves. That's right
for a PER-SEGMENT attr (silk, v_score — each independently true of
the segment it lands on) but wrong for a CLOSED-PROFILE SEED attr
(courtyard, mask_opening, mask_exclude, paste_aperture, pour,
keepout, board_cutout): the bake traces the WHOLE loop from ANY
entity carrying the attr, so two carriers on one loop emit the
region twice (two identical `FpPour`, two routed `FpCutout` on the
same board slot, ...). Exactly one entity on the loop — `line_a` —
may keep each seed attr after a split.
[test]

## Source
Lines 107–181 in `crates/oxide-sketch/src/split/tests/carry_over.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [carry_over](/crates/oxide-sketch/src/split/tests/carry_over.md) |
| calls | [line_sketch](/crates/oxide-sketch/src/split/tests/mod/line_sketch.md) |
| calls | [split_line](/crates/oxide-sketch/src/split/mod/split_line.md) |
