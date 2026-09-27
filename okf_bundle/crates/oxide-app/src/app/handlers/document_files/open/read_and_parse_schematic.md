---
okf_version: "0.2"
type: Function
title: read_and_parse_schematic
description: "Read + parse a `.snxsch` off the UI thread (the `spawn_blocking` body"
resource: crates/oxide-app/src/app/handlers/document_files/open.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/open/read_and_parse_schematic
language: rust
---

# read_and_parse_schematic

Read + parse a `.snxsch` off the UI thread (the `spawn_blocking` body

## Signature

```rust
fn read_and_parse_schematic(
    path: &std::path::Path,
) -> Result<oxide_types::schematic::SchematicSheet, String>
```

## Docstring

Read + parse a `.snxsch` off the UI thread (the `spawn_blocking` body
for `open_schematic_file`). Stringifies the full `anyhow` context
chain so the error survives the `Task::perform` boundary (`Message`
is `Clone`; `anyhow::Error` is not) — same shape as `HistoryLoaded`'s
`Result<_, String>`.

## Source
Lines 415–426 in `crates/oxide-app/src/app/handlers/document_files/open.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [open](/crates/oxide-app/src/app/handlers/document_files/open.md) |
| called_by | [open_schematic_file](/crates/oxide-app/src/app/handlers/document_files/open/open_schematic_file.md) |
