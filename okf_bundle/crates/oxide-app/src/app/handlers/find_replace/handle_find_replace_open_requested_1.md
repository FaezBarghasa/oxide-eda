---
okf_version: "0.2"
type: Function
title: handle_find_replace_open_requested
resource: crates/oxide-app/src/app/handlers/find_replace.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/find_replace/handle_find_replace_open_requested_1
language: rust
---

# handle_find_replace_open_requested

## Signature

```rust
pub(crate) fn handle_find_replace_open_requested(
        &mut self,
        replace_mode: bool,
    ) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Source
Lines 6–16 in `crates/oxide-app/src/app/handlers/find_replace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [find_replace](/crates/oxide-app/src/app/handlers/find_replace.md) |
