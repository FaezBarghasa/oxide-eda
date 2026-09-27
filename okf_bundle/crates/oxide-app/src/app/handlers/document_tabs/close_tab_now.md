---
okf_version: "0.2"
type: Function
title: close_tab_now
resource: crates/oxide-app/src/app/handlers/document_tabs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_tabs/close_tab_now
language: rust
---

# close_tab_now

## Signature

```rust
impl Oxide { pub(crate) fn close_tab_now(&mut self, idx: usize) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Source
Lines 97–190 in `crates/oxide-app/src/app/handlers/document_tabs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [document_tabs](/crates/oxide-app/src/app/handlers/document_tabs.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
