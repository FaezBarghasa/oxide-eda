---
okf_version: "0.2"
type: Function
title: incomplete_header
description: "The `# `-marked INCOMPLETE header prepended when `incomplete_note` is set"
resource: crates/oxide-output/src/netlist/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T19:55:41Z"
concept_id: crates/oxide-output/src/netlist/mod/incomplete_header
language: rust
---

# incomplete_header

The `# `-marked INCOMPLETE header prepended when `incomplete_note` is set

## Signature

```rust
fn incomplete_header(note: &[String]) -> String
```

## Docstring

The `# `-marked INCOMPLETE header prepended when `incomplete_note` is set
(#431). Leads with an unmissable warning, then lists one omitted page per
line. Deterministic: the caller passes `note` in a stable order, so two
exports of the same incomplete project diff cleanly.

## Source
Lines 80–107 in `crates/oxide-output/src/netlist/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [netlist](/crates/oxide-output/src/netlist/mod.md) |
| called_by | [render_listing](/crates/oxide-output/src/netlist/mod/render_listing.md) |
