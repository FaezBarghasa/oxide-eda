# test_support

## Classs

- [DenyNewFiles](DenyNewFiles.md) — RAII guard: while alive, `dir` rejects new-file creation. Restores

## Functions

- [allow](allow.md) — [cfg(unix)]
- [allow](allow_1.md) — [cfg(windows)]
- [deny](deny.md) — [cfg(unix)]
- [deny](deny_1.md) — Windows directories ignore the FILE_ATTRIBUTE_READONLY bit for new-file
- [drop](drop.md)
- [drop](drop_1.md)
- [has_stray_tmp](has_stray_tmp.md) — True if `dir` contains a leftover atomic-write temp sibling (`*.tmp`).
- [on](on.md)
- [on](on_1.md)
- [settle_deny](settle_deny.md) — `icacls`'s exit code is not proof the deny is enforced yet — under
