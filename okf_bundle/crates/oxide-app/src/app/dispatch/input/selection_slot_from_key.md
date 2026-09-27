---
okf_version: "0.2"
type: Function
title: selection_slot_from_key
description: "Which selection-memory slot a digit key names, if any."
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/selection_slot_from_key
language: rust
---

# selection_slot_from_key

Which selection-memory slot a digit key names, if any.

## Signature

```rust
fn selection_slot_from_key(key: &str) -> Option<usize>
```

## Docstring

Which selection-memory slot a digit key names, if any.

Moved here with its only consumer (`claim_selection_slots`) when the
keyboard branches left the subscription.

## Source
Lines 446–458 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
| called_by | [claim_selection_slots](/crates/oxide-app/src/app/dispatch/input/claim_selection_slots.md) |
