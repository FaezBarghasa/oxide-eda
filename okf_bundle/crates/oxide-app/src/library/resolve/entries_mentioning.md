---
okf_version: "0.2"
type: Function
title: entries_mentioning
description: Entries the Messages panel would be holding that mention
resource: crates/oxide-app/src/library/resolve.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/resolve/entries_mentioning
language: rust
---

# entries_mentioning

Entries the Messages panel would be holding that mention

## Signature

```rust
fn entries_mentioning(needle: &str) -> Vec<DiagnosticEntry>
```

## Docstring

Entries the Messages panel would be holding that mention
`needle`. Tests key on a per-test UUID so they stay independent
under libtest's default parallelism.

## Source
Lines 93–98 in `crates/oxide-app/src/library/resolve.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [resolve](/crates/oxide-app/src/library/resolve.md) |
| calls | [recent_entries](/crates/oxide-app/src/diagnostics/recent_entries.md) |
| called_by | [a_read_failure_reaches_the_messages_panel_before_collapsing_to_none](/crates/oxide-app/src/library/resolve/a_read_failure_reaches_the_messages_panel_before_collapsing_to_none.md) |
