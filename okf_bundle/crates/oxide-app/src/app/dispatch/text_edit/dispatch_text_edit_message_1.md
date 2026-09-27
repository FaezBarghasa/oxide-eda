---
okf_version: "0.2"
type: Function
title: dispatch_text_edit_message
resource: crates/oxide-app/src/app/dispatch/text_edit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/text_edit/dispatch_text_edit_message_1
language: rust
---

# dispatch_text_edit_message

## Signature

```rust
pub(super) fn dispatch_text_edit_message(&mut self, message: TextEditMsg) -> Task<Message>
```

## Visibility

- `pub(super)`

## Source
Lines 6–42 in `crates/oxide-app/src/app/dispatch/text_edit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text_edit](/crates/oxide-app/src/app/dispatch/text_edit.md) |
| calls | [escape_for_standard](/crates/oxide-app/src/schematic_runtime/text/escape_for_standard.md) |
| calls | [Label](/crates/oxide-types/src/schematic/sheet/Label.md) |
| calls | [TextNote](/crates/oxide-types/src/schematic/sheet/TextNote.md) |
