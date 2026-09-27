---
okf_version: "0.2"
type: Module
title: manifest
description: "`library.toml` schema for `*.snxlib/` directories. Mirrors v0.9-library-plan.md §13"
resource: crates/oxide-library/src/manifest.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/manifest
language: rust
---

# manifest

`library.toml` schema for `*.snxlib/` directories. Mirrors v0.9-library-plan.md §13

## Docstring

`library.toml` schema for `*.snxlib/` directories. Mirrors v0.9-library-plan.md §13
and `v0.9-refactor-2-plan.md` §3 (Altium DBLib model).

Adds the `[[tables]]` config section per `v0.9-refactor-2-plan.md` §6
step 1.5: a class with no override gets its own table (`<class>s.tsv`).
Explicit overrides let multiple classes share a table or rename the
filename.

## Relationships

| Type | Target |
|------|--------|
| related | [Manifest](/crates/oxide-library/src/manifest/Manifest.md) |
| related | [LibraryMeta](/crates/oxide-library/src/manifest/LibraryMeta.md) |
| related | [LibraryMode](/crates/oxide-library/src/manifest/LibraryMode.md) |
| related | [WorkflowConfig](/crates/oxide-library/src/manifest/WorkflowConfig.md) |
| related | [WorkflowMode](/crates/oxide-library/src/manifest/WorkflowMode.md) |
| related | [default_reviewers_required](/crates/oxide-library/src/manifest/default_reviewers_required.md) |
| related | [default_auto_promote](/crates/oxide-library/src/manifest/default_auto_promote.md) |
| related | [default](/crates/oxide-library/src/manifest/default.md) |
| related | [default](/crates/oxide-library/src/manifest/default.md) |
| related | [UsersConfig](/crates/oxide-library/src/manifest/UsersConfig.md) |
| related | [default_role](/crates/oxide-library/src/manifest/default_role.md) |
| related | [UserEntry](/crates/oxide-library/src/manifest/UserEntry.md) |
| related | [TableConfig](/crates/oxide-library/src/manifest/TableConfig.md) |
| related | [parse](/crates/oxide-library/src/manifest/parse.md) |
| related | [write](/crates/oxide-library/src/manifest/write.md) |
| related | [tables](/crates/oxide-library/src/manifest/tables.md) |
| related | [table_for_class](/crates/oxide-library/src/manifest/table_for_class.md) |
| related | [parse](/crates/oxide-library/src/manifest/parse.md) |
| related | [write](/crates/oxide-library/src/manifest/write.md) |
| related | [tables](/crates/oxide-library/src/manifest/tables.md) |
| related | [table_for_class](/crates/oxide-library/src/manifest/table_for_class.md) |
| related | [parses_local_git_manifest](/crates/oxide-library/src/manifest/parses_local_git_manifest.md) |
| related | [workflow_mode_round_trips](/crates/oxide-library/src/manifest/workflow_mode_round_trips.md) |
| related | [workflow_mode_defaults_to_personal_when_absent](/crates/oxide-library/src/manifest/workflow_mode_defaults_to_personal_when_absent.md) |
| related | [parses_database_manifest](/crates/oxide-library/src/manifest/parses_database_manifest.md) |
| related | [round_trip_preserves_workflow_defaults](/crates/oxide-library/src/manifest/round_trip_preserves_workflow_defaults.md) |
| related | [tables_overrides_round_trip_and_resolve](/crates/oxide-library/src/manifest/tables_overrides_round_trip_and_resolve.md) |
| related | [table_for_class_defaults_are_mechanical_plural](/crates/oxide-library/src/manifest/table_for_class_defaults_are_mechanical_plural.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
