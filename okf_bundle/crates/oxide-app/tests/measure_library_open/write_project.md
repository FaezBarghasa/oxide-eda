---
okf_version: "0.2"
type: Function
title: write_project
description: ── Project-level measurements ───────────────────────────────────────────
resource: crates/oxide-app/tests/measure_library_open.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:51:25Z"
concept_id: crates/oxide-app/tests/measure_library_open/write_project
language: rust
---

# write_project

── Project-level measurements ───────────────────────────────────────────

## Signature

```rust
fn write_project(dir: &Path, name: &str, libs: &[PathBuf]) -> PathBuf
```

## Docstring

── Project-level measurements ───────────────────────────────────────────

## Source
Lines 199–222 in `crates/oxide-app/tests/measure_library_open.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [measure_library_open](/crates/oxide-app/tests/measure_library_open.md) |
| called_by | [persist_project_by_id](/crates/oxide-app/src/app/handlers/document_files/save/persist_project_by_id.md) |
| called_by | [measure_library_open](/crates/oxide-app/tests/measure_library_open/measure_library_open.md) |
| called_by | [loaded_project_data_round_trips_via_write_then_parse](/crates/oxide-app/tests/regression/project/loaded_project_data_round_trips_via_write_then_parse.md) |
