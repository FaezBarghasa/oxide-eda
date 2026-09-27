---
okf_version: "0.2"
type: Function
title: records_mentioning
resource: crates/oxide-app/src/app/dir_listing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dir_listing/records_mentioning
language: rust
---

# records_mentioning

## Signature

```rust
fn records_mentioning(marker: &str) -> Vec<crate::diagnostics::DiagnosticEntry>
```

## Source
Lines 95–101 in `crates/oxide-app/src/app/dir_listing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dir_listing](/crates/oxide-app/src/app/dir_listing.md) |
| calls | [init_logging](/crates/oxide-app/src/diagnostics/init_logging.md) |
| calls | [recent_entries](/crates/oxide-app/src/diagnostics/recent_entries.md) |
| called_by | [an_unreadable_directory_reaches_the_messages_panel](/crates/oxide-app/src/app/dir_listing/an_unreadable_directory_reaches_the_messages_panel.md) |
