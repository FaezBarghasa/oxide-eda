---
okf_version: "0.2"
type: Function
title: load_preferred_order_at
description: Load preferred order from a specific path — extracted so tests can
resource: crates/oxide-app/src/library/settings/persistence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/settings/persistence/load_preferred_order_at
language: rust
---

# load_preferred_order_at

Load preferred order from a specific path — extracted so tests can

## Signature

```rust
pub fn load_preferred_order_at(path: &std::path::Path) -> Vec<DistributorSource>
```

## Visibility

- `pub`

## Docstring

Load preferred order from a specific path — extracted so tests can
hit the actual parse path without the `dirs::config_dir` dance.

## Source
Lines 123–151 in `crates/oxide-app/src/library/settings/persistence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [persistence](/crates/oxide-app/src/library/settings/persistence.md) |
| calls | [default_order](/crates/oxide-app/src/library/settings/persistence/default_order.md) |
| calls | [str_to_source](/crates/oxide-app/src/library/settings/persistence/str_to_source.md) |
| called_by | [corrupt_file_returns_default_order](/crates/oxide-app/src/library/settings/persistence/corrupt_file_returns_default_order.md) |
| called_by | [duplicates_are_collapsed](/crates/oxide-app/src/library/settings/persistence/duplicates_are_collapsed.md) |
| called_by | [empty_after_filter_falls_back_to_default](/crates/oxide-app/src/library/settings/persistence/empty_after_filter_falls_back_to_default.md) |
| called_by | [load_preferred_order](/crates/oxide-app/src/library/settings/persistence/load_preferred_order.md) |
| called_by | [missing_file_returns_default_order](/crates/oxide-app/src/library/settings/persistence/missing_file_returns_default_order.md) |
| called_by | [round_trip_writes_and_reads_back](/crates/oxide-app/src/library/settings/persistence/round_trip_writes_and_reads_back.md) |
| called_by | [unknown_distributor_strings_filtered_out](/crates/oxide-app/src/library/settings/persistence/unknown_distributor_strings_filtered_out.md) |
