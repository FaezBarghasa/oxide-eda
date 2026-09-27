---
okf_version: "0.2"
type: Function
title: save_preferred_order_at
description: Variant for tests / explicit paths.
resource: crates/oxide-app/src/library/settings/persistence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/settings/persistence/save_preferred_order_at
language: rust
---

# save_preferred_order_at

Variant for tests / explicit paths.

## Signature

```rust
pub fn save_preferred_order_at(
    path: &std::path::Path,
    order: &[DistributorSource],
) -> Result<(), String>
```

## Visibility

- `pub`

## Docstring

Variant for tests / explicit paths.

Crash-safe: [`oxide_types::atomic_io::atomic_write`] writes to a temp
sibling, fsyncs it and renames over the destination, so a crash mid-save
leaves the previously persisted order intact rather than a truncated file.
It also creates the parent directory, so no separate `create_dir_all`.

## Source
Lines 175–206 in `crates/oxide-app/src/library/settings/persistence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [persistence](/crates/oxide-app/src/library/settings/persistence.md) |
| calls | [atomic_write](/crates/oxide-app/src/app/dispatch/library/recovery/atomic_write.md) |
| called_by | [round_trip_writes_and_reads_back](/crates/oxide-app/src/library/settings/persistence/round_trip_writes_and_reads_back.md) |
| called_by | [save_preferred_order](/crates/oxide-app/src/library/settings/persistence/save_preferred_order.md) |
| called_by | [save_preferred_order_at_leaves_original_intact_when_write_fails](/crates/oxide-app/src/library/settings/persistence/save_preferred_order_at_leaves_original_intact_when_write_fails.md) |
