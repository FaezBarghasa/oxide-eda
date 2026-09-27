---
okf_version: "0.2"
type: Function
title: handle_netlist_cancel_incomplete
description: "#431 — \"Cancel\" on the netlist-incomplete prompt (or click-outside)."
resource: crates/oxide-app/src/app/handlers/menu/export/pdf_netlist.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/pdf_netlist/handle_netlist_cancel_incomplete
language: rust
---

# handle_netlist_cancel_incomplete

#431 — "Cancel" on the netlist-incomplete prompt (or click-outside).

## Signature

```rust
impl Oxide { pub(crate) fn handle_netlist_cancel_incomplete(&mut self) }
```

## Visibility

- `pub(crate)`

## Docstring

#431 — "Cancel" on the netlist-incomplete prompt (or click-outside).
Writes nothing; just clears the pending prompt.

## Source
Lines 267–269 in `crates/oxide-app/src/app/handlers/menu/export/pdf_netlist.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pdf_netlist](/crates/oxide-app/src/app/handlers/menu/export/pdf_netlist.md) |
