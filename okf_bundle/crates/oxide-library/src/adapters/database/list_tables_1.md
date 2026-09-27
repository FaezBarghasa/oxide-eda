---
okf_version: "0.2"
type: Function
title: list_tables
description: ── Row + table CRUD ─────────────────────────────────────────────────
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/list_tables_1
language: rust
---

# list_tables

── Row + table CRUD ─────────────────────────────────────────────────

## Signature

```rust
fn list_tables(&self) -> Result<Vec<String>, LibraryError>
```

## Docstring

── Row + table CRUD ─────────────────────────────────────────────────

The adapter forwards each method to its route on
`oxide-library-server`. The server-side DB schema lives in
`migrations/0005_tabular_components.sql`; the wire format is the
`ComponentRow` JSON serialisation defined in `component::ComponentRow`.

TODO(audit): mutating routes pass a commit message via
`x-oxide-message`, but the DB backend has no audit_log table yet —
the message currently shows up only in `tracing::info!` lines.
v0.9.x can add an `audit_log (library_id, row_id, actor, message,
occurred)` row per mutation when the workflow grows server-side
history beyond what the route handler logs surface.

## Source
Lines 296–310 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
