---
okf_version: "0.2"
type: Function
title: dispatch_export_message
description: "Export subsystem message handler (namespaced family, ADR-0001 D3)."
resource: crates/oxide-app/src/app/dispatch/document.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/document/dispatch_export_message
language: rust
---

# dispatch_export_message

Export subsystem message handler (namespaced family, ADR-0001 D3).

## Signature

```rust
impl Oxide { pub(crate) fn dispatch_export_message(&mut self, msg: ExportMsg) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Export subsystem message handler (namespaced family, ADR-0001 D3).
PDF / netlist / BOM export lifecycle plus the export-error modal
dismiss. Some arms delegate to helpers in
`app/handlers/menu/export.rs`.

## Source
Lines 147–178 in `crates/oxide-app/src/app/dispatch/document.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [document](/crates/oxide-app/src/app/dispatch/document.md) |
