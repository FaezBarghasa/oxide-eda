---
okf_version: "0.2"
type: Function
title: split_duplicated
description: "`Horizontal` / `Vertical` — duplicated onto both halves (the"
resource: crates/oxide-sketch/src/split/constraints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/split/constraints/split_duplicated
language: rust
---

# split_duplicated

`Horizontal` / `Vertical` — duplicated onto both halves (the

## Signature

```rust
fn split_duplicated(c: &Constraint, ctx: &SplitCtx) -> Option<Vec<Constraint>>
```

## Docstring

`Horizontal` / `Vertical` — duplicated onto both halves (the
original id keeps `line_a`, a freshly minted id covers `line_b`).

Both are absolute-frame, single-line predicates: pinning one
endpoint's y (or x) equal to the other's. The new mid Point has 2
fresh DOF the original 2-point Line never had. Duplicating spends 2
residuals total (1 per half): the copy on `line_a` pins `mid`'s
axis coordinate to `start`'s (the one genuinely new relation), and
the copy on `line_b` re-derives `end`'s axis coordinate from
`mid`'s — already implied once `line_a`'s copy holds, but harmless
to restate, and it is what keeps `line_b` independently straight if
`mid` is later dragged off both lines by an unrelated edit. Net DOF
goes from 3 free (the original Line's endpoints, one axis pinned)
to 4 free (both halves' endpoints, one axis pinned on each) — not
"exactly balanced" against the 2 new residuals, but rank 2 over the
2 new (`mid.x`, `mid.y`) variables with `mid.x` left free is
correct and cannot over-constrain by itself.

## Source
Lines 78–93 in `crates/oxide-sketch/src/split/constraints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints](/crates/oxide-sketch/src/split/constraints.md) |
| calls | [duplicate](/crates/oxide-sketch/src/split/constraints/duplicate.md) |
| called_by | [split_constraint](/crates/oxide-sketch/src/split/constraints/split_constraint.md) |
