# Golden snapshots

`commands.json` locks the stable command-id surface (`id` + `group` +
`category`) covered by `keymap::catalog::tests::command_id_surface_matches_golden_snapshot`
in `crates/oxide-app/src/keymap/catalog/mod.rs` — see oxide#276.

Regenerate ONLY for an intentional additive/aliased change to the catalog:

```
UPDATE_GOLDEN=1 cargo test -p oxide-app command_id_surface_matches_golden_snapshot
```

Never regenerate to make a rename/removal diff disappear — that's the
breakage this test exists to catch. Add an alias + a deprecation note
instead.
