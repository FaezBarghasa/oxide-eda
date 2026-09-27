---
okf_version: "0.2"
type: Function
title: dispatch_print_preview_message
description: "Print-preview modal message handler (namespaced family, ADR-0001 D3)."
resource: crates/oxide-app/src/app/dispatch/document.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/document/dispatch_print_preview_message
language: rust
---

# dispatch_print_preview_message

Print-preview modal message handler (namespaced family, ADR-0001 D3).

## Signature

```rust
impl Oxide { pub(crate) fn dispatch_print_preview_message(&mut self, msg: PrintPreviewMsg) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Print-preview modal message handler (namespaced family, ADR-0001 D3).

## Source
Lines 286–507 in `crates/oxide-app/src/app/dispatch/document.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [document](/crates/oxide-app/src/app/dispatch/document.md) |
