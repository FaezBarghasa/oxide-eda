---
okf_version: "0.2"
type: Function
title: annotate_reset_confirm_outranks_annotate_dialog
description: "#514 — the reset confirm is a child of the annotate dialog with"
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/annotate_reset_confirm_outranks_annotate_dialog
language: rust
---

# annotate_reset_confirm_outranks_annotate_dialog

#514 — the reset confirm is a child of the annotate dialog with

## Signature

```rust
fn annotate_reset_confirm_outranks_annotate_dialog()
```

## Decorators

- `test`

## Docstring

#514 — the reset confirm is a child of the annotate dialog with
its own distinct Cancel message; getting the ordering backwards
would resolve Esc to `CloseDialog` and close the parent out from
under the confirm.
[test]

## Source
Lines 1065–1075 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
