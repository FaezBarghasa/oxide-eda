# persistence

## Classs

- [DistributorApisSection](DistributorApisSection.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [DistributorsConfig](DistributorsConfig.md) — On-disk shape. `serde(default)` lets older / partial files load

## Functions

- [config_path](config_path.md) — Resolve `<config_dir>/oxide/distributors.toml`. Returns `None`
- [config_path_for_dir](config_path_for_dir.md) — Test-friendly variant — same layout, but rooted under the supplied
- [corrupt_file_returns_default_order](corrupt_file_returns_default_order.md) — [test]
- [default](default.md)
- [default](default_1.md)
- [default_order](default_order.md) — Default order matches `DistributorSettings::default()`.
- [default_order_strs](default_order_strs.md)
- [duplicates_are_collapsed](duplicates_are_collapsed.md) — [test]
- [empty_after_filter_falls_back_to_default](empty_after_filter_falls_back_to_default.md) — [test]
- [load_preferred_order](load_preferred_order.md) — Load the persisted preferred-order list. Returns the v0.9-library-plan.md
- [load_preferred_order_at](load_preferred_order_at.md) — Load preferred order from a specific path — extracted so tests can
- [missing_file_returns_default_order](missing_file_returns_default_order.md) — [test]
- [round_trip_writes_and_reads_back](round_trip_writes_and_reads_back.md) — [test]
- [save_preferred_order](save_preferred_order.md) — Persist the preferred-order list. Errors warn through `tracing` and
- [save_preferred_order_at](save_preferred_order_at.md) — Variant for tests / explicit paths.
- [save_preferred_order_at_leaves_original_intact_when_write_fails](save_preferred_order_at_leaves_original_intact_when_write_fails.md) — `save_preferred_order_at` must go through `atomic_write`, not
- [source_str_round_trip_covers_every_variant](source_str_round_trip_covers_every_variant.md) — [test]
- [source_to_str](source_to_str.md) — Wire-name for a distributor source — kept in this module so the
- [str_to_source](str_to_source.md)
- [unknown_distributor_strings_filtered_out](unknown_distributor_strings_filtered_out.md) — [test]
