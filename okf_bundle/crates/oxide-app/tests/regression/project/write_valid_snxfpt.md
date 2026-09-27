---
okf_version: "0.2"
type: Function
title: write_valid_snxfpt
description: "Write a valid single-footprint `.snxfpt` envelope to `path`."
resource: crates/oxide-app/tests/regression/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:39Z"
concept_id: crates/oxide-app/tests/regression/project/write_valid_snxfpt
language: rust
---

# write_valid_snxfpt

Write a valid single-footprint `.snxfpt` envelope to `path`.

## Signature

```rust
fn write_valid_snxfpt(path: &Path, name: &str)
```

## Docstring

Write a valid single-footprint `.snxfpt` envelope to `path`.
Uses the real `FootprintFile::to_toml_string` so the file parses
cleanly — the test then proves the *gate* blocks the open, not a
parse failure (which would be a false green).

## Source
Lines 868–873 in `crates/oxide-app/tests/regression/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-app/tests/regression/project.md) |
| called_by | [opening_snxfpt_does_not_create_editable_tab_when_gated](/crates/oxide-app/tests/regression/project/opening_snxfpt_does_not_create_editable_tab_when_gated.md) |
