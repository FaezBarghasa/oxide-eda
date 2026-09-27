---
okf_version: "0.2"
type: Module
title: netlist
description: Netlist export.
resource: crates/oxide-output/src/netlist/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T19:55:41Z"
concept_id: crates/oxide-output/src/netlist/mod
language: rust
---

# netlist

Netlist export.

## Docstring

Netlist export.

The Standard-format `.net` S-expression emitter that previously lived
here was split out as part of the issue #62 Apache-clean cutover.
It moves to the optional `oxide-standard-import` GPL-3.0 companion
repository alongside the rest of the Standard I/O codepaths.

What stays here is the *input side* (ADR-0002 D7): the exporter reads the
authoritative [`Netlist`](oxide_types::net::Netlist) off [`ExportContext`]
— derived once by the app through `oxide_net::build_project_netlist` — so
future Oxide-native emitters (XML, Spice, …) land against the contract
instead of re-deriving connectivity. The interim emitter writes a plain,
deterministic net listing; the Standard `.net` format is issue #62.

## Relationships

| Type | Target |
|------|--------|
| related | [NetlistExporter](/crates/oxide-output/src/netlist/mod/NetlistExporter.md) |
| related | [NetlistOptions](/crates/oxide-output/src/netlist/mod/NetlistOptions.md) |
| related | [NetlistOutput](/crates/oxide-output/src/netlist/mod/NetlistOutput.md) |
| related | [NetlistError](/crates/oxide-output/src/netlist/mod/NetlistError.md) |
| related | [export](/crates/oxide-output/src/netlist/mod/export.md) |
| related | [export](/crates/oxide-output/src/netlist/mod/export.md) |
| related | [incomplete_header](/crates/oxide-output/src/netlist/mod/incomplete_header.md) |
| related | [render_listing](/crates/oxide-output/src/netlist/mod/render_listing.md) |
| related | [ctx_with](/crates/oxide-output/src/netlist/mod/ctx_with.md) |
| related | [export_errors_without_a_netlist](/crates/oxide-output/src/netlist/mod/export_errors_without_a_netlist.md) |
| related | [export_renders_the_netlist_deterministically](/crates/oxide-output/src/netlist/mod/export_renders_the_netlist_deterministically.md) |
| related | [one_net](/crates/oxide-output/src/netlist/mod/one_net.md) |
| related | [incomplete_note_prepends_a_comment_header_naming_omitted_pages](/crates/oxide-output/src/netlist/mod/incomplete_note_prepends_a_comment_header_naming_omitted_pages.md) |
| related | [without_a_note_the_output_is_byte_identical_to_the_plain_dump](/crates/oxide-output/src/netlist/mod/without_a_note_the_output_is_byte_identical_to_the_plain_dump.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |
