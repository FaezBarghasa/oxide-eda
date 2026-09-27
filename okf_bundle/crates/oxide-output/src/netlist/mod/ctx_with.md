---
okf_version: "0.2"
type: Function
title: ctx_with
resource: crates/oxide-output/src/netlist/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T19:55:41Z"
concept_id: crates/oxide-output/src/netlist/mod/ctx_with
language: rust
---

# ctx_with

## Signature

```rust
fn ctx_with(netlist: Option<Netlist>) -> ExportContext
```

## Source
Lines 142–148 in `crates/oxide-output/src/netlist/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [netlist](/crates/oxide-output/src/netlist/mod.md) |
| called_by | [export_errors_without_a_netlist](/crates/oxide-output/src/netlist/mod/export_errors_without_a_netlist.md) |
| called_by | [export_renders_the_netlist_deterministically](/crates/oxide-output/src/netlist/mod/export_renders_the_netlist_deterministically.md) |
| called_by | [incomplete_note_prepends_a_comment_header_naming_omitted_pages](/crates/oxide-output/src/netlist/mod/incomplete_note_prepends_a_comment_header_naming_omitted_pages.md) |
| called_by | [without_a_note_the_output_is_byte_identical_to_the_plain_dump](/crates/oxide-output/src/netlist/mod/without_a_note_the_output_is_byte_identical_to_the_plain_dump.md) |
