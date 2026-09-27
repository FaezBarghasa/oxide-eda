---
okf_version: "0.2"
type: Function
title: two_edits_then_two_undos_restores_both
description: "The failure a user met: two edits in, two Undos must get back to the"
resource: crates/oxide-app/tests/regression/undo_marker_divergence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/undo_marker_divergence/two_edits_then_two_undos_restores_both
language: rust
---

# two_edits_then_two_undos_restores_both

The failure a user met: two edits in, two Undos must get back to the

## Signature

```rust
fn two_edits_then_two_undos_restores_both()
```

## Decorators

- `test`

## Docstring

The failure a user met: two edits in, two Undos must get back to the
starting document. The second Undo used to do nothing, so the deleted
symbol never came back.
[test]

## Source
Lines 215–259 in `crates/oxide-app/tests/regression/undo_marker_divergence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [undo_marker_divergence](/crates/oxide-app/tests/regression/undo_marker_divergence.md) |
| calls | [one_gateway_edit_then_one_bypassing_edit](/crates/oxide-app/tests/regression/undo_marker_divergence/one_gateway_edit_then_one_bypassing_edit.md) |
