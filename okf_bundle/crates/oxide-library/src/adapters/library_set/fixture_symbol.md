---
okf_version: "0.2"
type: Function
title: fixture_symbol
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/fixture_symbol
language: rust
---

# fixture_symbol

## Signature

```rust
fn fixture_symbol(name: &str) -> Symbol
```

## Source
Lines 442–444 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
| called_by | [mount_and_resolve_symbol](/crates/oxide-library/src/adapters/library_set/mount_and_resolve_symbol.md) |
| called_by | [read_failure_is_reported_not_reported_as_a_missing_uuid](/crates/oxide-library/src/adapters/library_set/read_failure_is_reported_not_reported_as_a_missing_uuid.md) |
| called_by | [remount_replaces_previous_adapter_and_returns_old](/crates/oxide-library/src/adapters/library_set/remount_replaces_previous_adapter_and_returns_old.md) |
| called_by | [unmount_returns_adapter_and_drops_resolution](/crates/oxide-library/src/adapters/library_set/unmount_returns_adapter_and_drops_resolution.md) |
| called_by | [unresolved_refs_filters_to_only_missing](/crates/oxide-library/src/adapters/library_set/unresolved_refs_filters_to_only_missing.md) |
