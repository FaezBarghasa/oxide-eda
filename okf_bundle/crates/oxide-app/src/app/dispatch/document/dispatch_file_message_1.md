---
okf_version: "0.2"
type: Function
title: dispatch_file_message
description: "File / save message handler (namespaced family, ADR-0001 D3)."
resource: crates/oxide-app/src/app/dispatch/document.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/document/dispatch_file_message_1
language: rust
---

# dispatch_file_message

File / save message handler (namespaced family, ADR-0001 D3).

## Signature

```rust
pub(crate) fn dispatch_file_message(&mut self, msg: FileMsg) -> Task<Message>
```

## Visibility

- `pub(crate)`

## Docstring

File / save message handler (namespaced family, ADR-0001 D3).
Covers open / new-project / save / save-as / save-primitive-as
plus the async `SchematicOpenFinished` / `PcbOpenFinished`
completions.

## Source
Lines 84–141 in `crates/oxide-app/src/app/dispatch/document.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [document](/crates/oxide-app/src/app/dispatch/document.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [log_error](/crates/oxide-app/src/diagnostics/log_error.md) |
