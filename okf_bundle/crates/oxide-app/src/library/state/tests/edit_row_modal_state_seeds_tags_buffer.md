---
okf_version: "0.2"
type: Function
title: edit_row_modal_state_seeds_tags_buffer
description: "`EditRowModalState::new` seeds `tags_buf` from"
resource: crates/oxide-app/src/library/state/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/state/tests/edit_row_modal_state_seeds_tags_buffer
language: rust
---

# edit_row_modal_state_seeds_tags_buffer

`EditRowModalState::new` seeds `tags_buf` from

## Signature

```rust
fn edit_row_modal_state_seeds_tags_buffer()
```

## Decorators

- `test`

## Docstring

`EditRowModalState::new` seeds `tags_buf` from
`parameters["tags"]` so the modal opens with the existing
tags rendered in the input — Stage 18 lifecycle/tag UX.
[test]

## Source
Lines 180–222 in `crates/oxide-app/src/library/state/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/state/tests.md) |
