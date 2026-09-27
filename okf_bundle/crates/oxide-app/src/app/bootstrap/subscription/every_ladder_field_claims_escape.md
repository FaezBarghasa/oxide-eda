---
okf_version: "0.2"
type: Function
title: every_ladder_field_claims_escape
description: "#514 — the actual reported symptom, generalized: Esc must never"
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/every_ladder_field_claims_escape
language: rust
---

# every_ladder_field_claims_escape

#514 — the actual reported symptom, generalized: Esc must never

## Signature

```rust
fn every_ladder_field_claims_escape()
```

## Decorators

- `test`

## Docstring

#514 — the actual reported symptom, generalized: Esc must never
reach `Message::EscapePressed` (the tool reset) while a modal
represented on the ladder is open.

What this test mechanically guarantees, precisely (no more, no
less):

1. **Every field on `OpenOverlays`, of any type, is accounted for.**
`open_overlays_field_names!`'s identifier-list argument is
destructured against `OpenOverlays::default()` with no `..` rest
pattern, so add, remove, or rename ANY field — bool or not — and
this test fails to COMPILE ("pattern does not mention field
`…`") until the list is updated. `recovery_kind` is not
special-cased out of this list — it goes through the exact same
macro as every bool field, which is what closed the previous
hole (a hand-written non-bool tail that carried no rung
requirement).
2. **Every named field claims Esc, by NAME.** The `assert_eq!`
below is a SET-EQUALITY check of NAMES — the macro's field-name
list vs. `setters`' (deduplicated) first elements — not a
count. There is deliberately no opt-out list: an `EXCLUDED`
escape hatch existed while `palette_open` and
`keymap_recorder_open` sat on this struct, and it let a field
skip guarantees 3 and 4 entirely just by being named in it.
Those two fields were write-only leftovers of the pre-#557
subscription closure — their real Esc handling is
`claim_command_palette` / `claim_keymap_recorder` in
`app/dispatch/input.rs` — so both they and the hatch are gone,
and a new field now has exactly one way to satisfy this test:
wire it a rung. This catches a name that isn't a
real field at all (a typo). It does NOT, by itself, catch a
correctly-labeled entry whose closure mutates the WRONG field:
for a genuinely new struct field `new_field_open`, the entry
`("new_field_open", |o| o.prefs_open = true)` passes this check
cleanly, because `"new_field_open"` IS a real field (the
exhaustive destructure in point 1 forces it into the macro's
list) and the label matches. Guarantee 4 below is what catches
that case.
3. **Every "claims Esc" field actually resolves to `Some(_)`.** The
first loop runs each `setters` closure through `only` and
asserts non-`None`. This alone does not catch the mis-wire in
point 2 either — `prefs_open` really does resolve to `Some(_)`.
4. **No two `setters` entries resolve to the same `Message`.** The
second loop asserts distinctness (by `Debug` string, since
`Message` isn't `PartialEq`) across every entry's resolved
message. A closure that (accidentally or via copy-paste) sets
the wrong field produces a DUPLICATE of that field's real entry
— e.g. the `new_field_open` mis-wire above would resolve to
`PreferencesMsg::Close` twice, once under each of two different
names — which this catches even though 2 and 3 both pass it.

What it does NOT guarantee: which exact `Message` variant a rung
returns (that's `every_modal_claims_escape`, above), or relative
ordering between rungs (that's the dedicated precedence tests).
[test]

## Source
Lines 1377–1521 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
