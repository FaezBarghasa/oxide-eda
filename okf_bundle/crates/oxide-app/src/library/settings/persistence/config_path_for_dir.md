---
okf_version: "0.2"
type: Function
title: config_path_for_dir
description: "Test-friendly variant — same layout, but rooted under the supplied"
resource: crates/oxide-app/src/library/settings/persistence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/settings/persistence/config_path_for_dir
language: rust
---

# config_path_for_dir

Test-friendly variant — same layout, but rooted under the supplied

## Signature

```rust
pub fn config_path_for_dir(base: &std::path::Path) -> PathBuf
```

## Visibility

- `pub`

## Docstring

Test-friendly variant — same layout, but rooted under the supplied
directory. Lets unit tests round-trip without touching the real
per-user config dir.

## Source
Lines 108–110 in `crates/oxide-app/src/library/settings/persistence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [persistence](/crates/oxide-app/src/library/settings/persistence.md) |
| calls | [config_root_for_dir](/crates/oxide-app/src/config_root/config_root_for_dir.md) |
| called_by | [corrupt_file_returns_default_order](/crates/oxide-app/src/library/settings/persistence/corrupt_file_returns_default_order.md) |
| called_by | [duplicates_are_collapsed](/crates/oxide-app/src/library/settings/persistence/duplicates_are_collapsed.md) |
| called_by | [empty_after_filter_falls_back_to_default](/crates/oxide-app/src/library/settings/persistence/empty_after_filter_falls_back_to_default.md) |
| called_by | [missing_file_returns_default_order](/crates/oxide-app/src/library/settings/persistence/missing_file_returns_default_order.md) |
| called_by | [round_trip_writes_and_reads_back](/crates/oxide-app/src/library/settings/persistence/round_trip_writes_and_reads_back.md) |
| called_by | [save_preferred_order_at_leaves_original_intact_when_write_fails](/crates/oxide-app/src/library/settings/persistence/save_preferred_order_at_leaves_original_intact_when_write_fails.md) |
| called_by | [unknown_distributor_strings_filtered_out](/crates/oxide-app/src/library/settings/persistence/unknown_distributor_strings_filtered_out.md) |
