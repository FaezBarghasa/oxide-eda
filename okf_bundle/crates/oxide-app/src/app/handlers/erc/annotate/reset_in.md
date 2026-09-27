---
okf_version: "0.2"
type: Function
title: reset_in
description: "Reset helper: for each symbol whose current reference is in"
resource: crates/oxide-app/src/app/handlers/erc/annotate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/erc/annotate/reset_in
language: rust
---

# reset_in

Reset helper: for each symbol whose current reference is in

## Signature

```rust
impl Oxide { fn reset_in(
            sheet: &mut oxide_types::schematic::SchematicSheet,
            dupes: &HashSet<String>,
        ) -> bool }
```

## Docstring

Reset helper: for each symbol whose current reference is in
the duplicates set, reset to `{prefix}?`. Returns whether
anything changed in the sheet.

## Source
Lines 263–285 in `crates/oxide-app/src/app/handlers/erc/annotate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [annotate](/crates/oxide-app/src/app/handlers/erc/annotate.md) |
