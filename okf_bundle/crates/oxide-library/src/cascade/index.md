# cascade

## Classs

- [CascadeReport](CascadeReport.md) — Outcome of one cascade pass — which rows were silently auto-bumped
- [PrimitiveKindTag](PrimitiveKindTag.md) — Internal dispatcher — picks the right binding column and shared

## Functions

- [apply_cascade_bump](apply_cascade_bump.md)
- [cascade_after_footprint_save](cascade_after_footprint_save.md) — Run the cascade after a `save_footprint(fp, _)` succeeded.
- [cascade_after_save](cascade_after_save.md)
- [cascade_after_sim_save](cascade_after_sim_save.md) — Run the cascade after a `save_sim(sm, _)` succeeded.
- [cascade_after_symbol_save](cascade_after_symbol_save.md) — Run the cascade after a `save_symbol(sym, _)` succeeded.
- [cascade_commit_message](cascade_commit_message.md) — Synthesise the git commit message for a cascade-driven row update.
- [cascade_report_default_is_empty](cascade_report_default_is_empty.md) — [test]
- [is_empty](is_empty.md)
- [is_empty](is_empty_1.md)
- [label](label.md)
- [label](label_1.md)
- [patch_bump](patch_bump.md) — Patch-bump a semver-style `X.Y.Z` string. Returns `<old>.1`
- [patch_bump_falls_back_on_garbage](patch_bump_falls_back_on_garbage.md) — [test]
- [patch_bump_increments_clean_semver](patch_bump_increments_clean_semver.md) — [test]
- [row_binds_to](row_binds_to.md)
- [short_uuid](short_uuid.md)
