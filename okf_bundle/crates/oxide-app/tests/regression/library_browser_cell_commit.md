---
okf_version: "0.2"
type: Module
title: library_browser_cell_commit
description: "#599 — an inline Library Browser cell commit must not retype a"
resource: crates/oxide-app/tests/regression/library_browser_cell_commit.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_browser_cell_commit
language: rust
---

# library_browser_cell_commit

#599 — an inline Library Browser cell commit must not retype a

## Docstring

#599 — an inline Library Browser cell commit must not retype a
parameter.

A cell holding `Measurement { 3.3, "V" }` edited to something that is
not an `f64` used to be written to the library as
`ParamValue::Text(buffer)`: the cell rendered the typed text so it
looked committed, while the numeric type and the unit that every
numeric facet filter, range query, sort and BOM column keys on were
gone from the `.snxlib` on disk.

This drives the real dispatcher against a real `LocalGitAdapter`
library and then re-reads the row from disk through a fresh adapter,
so it fails if the refusal is removed.

## Relationships

| Type | Target |
|------|--------|
| related | [library_with_a_measurement_row](/crates/oxide-app/tests/regression/library_browser_cell_commit/library_with_a_measurement_row.md) |
| related | [param_on_disk](/crates/oxide-app/tests/regression/library_browser_cell_commit/param_on_disk.md) |
| related | [an_unparseable_buffer_does_not_retype_a_measurement_parameter](/crates/oxide-app/tests/regression/library_browser_cell_commit/an_unparseable_buffer_does_not_retype_a_measurement_parameter.md) |
| related | [a_refused_cell_commit_reaches_the_messages_panel](/crates/oxide-app/tests/regression/library_browser_cell_commit/a_refused_cell_commit_reaches_the_messages_panel.md) |
