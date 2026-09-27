---
okf_version: "0.2"
type: Function
title: handle_netlist_export_anyway
description: "#431 — \"Export anyway (incomplete)\". The user explicitly chose to ship"
resource: crates/oxide-app/src/app/handlers/menu/export/pdf_netlist.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/pdf_netlist/handle_netlist_export_anyway
language: rust
---

# handle_netlist_export_anyway

#431 — "Export anyway (incomplete)". The user explicitly chose to ship

## Signature

```rust
impl Oxide { pub(crate) fn handle_netlist_export_anyway(&mut self) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

#431 — "Export anyway (incomplete)". The user explicitly chose to ship
the partial netlist. Re-derives the export scope, writes the
best-available (root-reachable) netlist to the pending path WITH an
INCOMPLETE header comment listing the omitted pages, then clears the
prompt. The header is non-negotiable: a partial `.net` NEVER reaches
disk without the incompleteness recorded in the file itself, so a
downstream PCB import can see it is partial.

## Source
Lines 223–263 in `crates/oxide-app/src/app/handlers/menu/export/pdf_netlist.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pdf_netlist](/crates/oxide-app/src/app/handlers/menu/export/pdf_netlist.md) |
