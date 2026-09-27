---
okf_version: "0.2"
type: Function
title: delete_confirm_ranks_below_the_primitive_picker_and_above_the_library_picker
description: Order is load-bearing — the deepest (topmost-painted) modal wins.
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/delete_confirm_ranks_below_the_primitive_picker_and_above_the_library_picker
language: rust
---

# delete_confirm_ranks_below_the_primitive_picker_and_above_the_library_picker

Order is load-bearing — the deepest (topmost-painted) modal wins.

## Signature

```rust
fn delete_confirm_ranks_below_the_primitive_picker_and_above_the_library_picker()
```

## Decorators

- `test`

## Docstring

Order is load-bearing — the deepest (topmost-painted) modal wins.
`passive_calculator_overlay` paints at `view/mod.rs:789`,
`find_replace_overlay` at `:787` (earlier, i.e. underneath) and
`preferences_overlay` at `:786` (earliest of the three) — so with
all three flags set, the passive calculator must win. This is a
KNOWN reversal from trunk (where `find_replace_open` used to win):
that was a latent ordering bug — find_replace outranked a modal
that visually painted on top of it — fixed as a side effect of
deriving the whole ladder from paint order instead of hand-ordering
it (see the doc comment on `escape_message`).
#535 — the rung that could not exist before the ladder moved to
`update`, pinned in both directions.

Membership is already forced by `every_ladder_field_claims_escape`;
this is about *position*. `delete_confirm_overlay` paints between
`edit_row_modal_overlay` and `primitive_picker_overlay`, so the rung
must lose to the primitive picker and beat the library picker. Get
this backwards and Esc dismisses a card underneath the one the user
is looking at.
[test]

## Source
Lines 1010–1044 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
