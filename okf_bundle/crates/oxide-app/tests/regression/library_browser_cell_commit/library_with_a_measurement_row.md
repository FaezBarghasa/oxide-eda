---
okf_version: "0.2"
type: Function
title: library_with_a_measurement_row
description: "A `.snxlib` with one row whose `Supply Voltage` is a typed"
resource: crates/oxide-app/tests/regression/library_browser_cell_commit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_browser_cell_commit/library_with_a_measurement_row
language: rust
---

# library_with_a_measurement_row

A `.snxlib` with one row whose `Supply Voltage` is a typed

## Signature

```rust
fn library_with_a_measurement_row(dir: &std::path::Path) -> (PathBuf, RowId)
```

## Docstring

A `.snxlib` with one row whose `Supply Voltage` is a typed
measurement. Version control off — `LocalGitAdapter::open` treats a
missing `.git/` as "no version control" and every mutation is a
best-effort commit that no-ops, which keeps the fixture cheap while
still writing real bytes through the real writer.

## Source
Lines 41–105 in `crates/oxide-app/tests/regression/library_browser_cell_commit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_browser_cell_commit](/crates/oxide-app/tests/regression/library_browser_cell_commit.md) |
| calls | [library_id](/crates/oxide-library/src/adapter/library_id.md) |
| called_by | [a_refused_cell_commit_reaches_the_messages_panel](/crates/oxide-app/tests/regression/library_browser_cell_commit/a_refused_cell_commit_reaches_the_messages_panel.md) |
| called_by | [an_unparseable_buffer_does_not_retype_a_measurement_parameter](/crates/oxide-app/tests/regression/library_browser_cell_commit/an_unparseable_buffer_does_not_retype_a_measurement_parameter.md) |
