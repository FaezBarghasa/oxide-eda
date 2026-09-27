---
okf_version: "0.2"
type: Module
title: lib
description: Oxide component library subsystem (v0.9-refactor-2 — DBLib model).
resource: crates/oxide-library/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:59:15Z"
concept_id: crates/oxide-library/src/lib
language: rust
---

# lib

Oxide component library subsystem (v0.9-refactor-2 — DBLib model).

## Docstring

Oxide component library subsystem (v0.9-refactor-2 — DBLib model).

Per `v0.9-refactor-2-plan.md`, components are **rows in TSV/JSONB
tables**, not files. Each row references reusable primitives (`Symbol`,
`Footprint`, `SimModel`) by `(library_id, uuid)` tuples ([`PrimitiveRef`])
instead of embedding their geometry. Symbols / footprints / sims stay as
standalone editable primitive files; the row binds them with metadata.

## Relationships

| Type | Target |
|------|--------|
| related | [enable_project_version_control](/crates/oxide-library/src/lib/enable_project_version_control.md) |
| related | [project_file_history](/crates/oxide-library/src/lib/project_file_history.md) |
| related | [commit_to_history_entry](/crates/oxide-library/src/lib/commit_to_history_entry.md) |
| related | [commit_touches_path](/crates/oxide-library/src/lib/commit_touches_path.md) |
| related | [crate_compiles](/crates/oxide-library/src/lib/crate_compiles.md) |
