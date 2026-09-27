---
okf_version: "0.2"
type: Function
title: handle_annotate_order_changed
resource: crates/oxide-app/src/app/handlers/erc/annotate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/erc/annotate/handle_annotate_order_changed_1
language: rust
---

# handle_annotate_order_changed

## Signature

```rust
pub(crate) fn handle_annotate_order_changed(
        &mut self,
        order: super::super::super::state::AnnotateOrder,
    ) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 400–406 in `crates/oxide-app/src/app/handlers/erc/annotate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [annotate](/crates/oxide-app/src/app/handlers/erc/annotate.md) |
