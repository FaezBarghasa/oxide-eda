---
okf_version: "0.2"
type: Function
title: render_listing
description: "A plain, deterministic net listing: one `net <id> \"<name>\"` line per net (in"
resource: crates/oxide-output/src/netlist/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T19:55:41Z"
concept_id: crates/oxide-output/src/netlist/mod/render_listing
language: rust
---

# render_listing

A plain, deterministic net listing: one `net <id> "<name>"` line per net (in

## Signature

```rust
fn render_listing(
    netlist: &oxide_types::net::Netlist,
    incomplete_note: Option<&[String]>,
) -> Vec<u8>
```

## Docstring

A plain, deterministic net listing: one `net <id> "<name>"` line per net (in
the netlist's stable id order) followed by its `reference.pin` terminals (in
the order the derivation already sorted them). Not the Standard `.net`
format — that emitter is issue #62 — but a stable, contract-driven dump.

When `incomplete_note` is `Some`, an INCOMPLETE header comment block (see
[`incomplete_header`]) is prepended before the banner (#431); when `None`
the output is byte-identical to the plain dump.

## Source
Lines 117–134 in `crates/oxide-output/src/netlist/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [netlist](/crates/oxide-output/src/netlist/mod.md) |
| calls | [incomplete_header](/crates/oxide-output/src/netlist/mod/incomplete_header.md) |
| called_by | [export](/crates/oxide-output/src/netlist/mod/export.md) |
