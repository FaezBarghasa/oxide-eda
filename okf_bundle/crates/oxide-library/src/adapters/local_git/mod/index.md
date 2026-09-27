# mod

## Classs

- [LibraryInitOptions](LibraryInitOptions.md) — Library-create options threaded through [`LocalGitAdapter::init`].
- [LocalGitAdapter](LocalGitAdapter.md) — Adapter over a `.snxlib`-file-rooted directory + git repo.

## Functions

- [file_path_buf](file_path_buf.md) — Borrow the absolute path to the `.snxlib` file itself.
- [file_path_buf](file_path_buf_1.md) — Borrow the absolute path to the `.snxlib` file itself.
- [fixture_snx_manifest](fixture_snx_manifest.md)
- [fixture_snxlib_path](fixture_snxlib_path.md)
- [fixture_symbol](fixture_symbol.md)
- [init](init.md) — Initialise a fresh library at `file_path`. The path must end
- [init](init_1.md) — Initialise a fresh library at `file_path`. The path must end
- [init_creates_snxlib_file_and_repo](init_creates_snxlib_file_and_repo.md) — [test]
- [lfs_off_skips_gitattributes](lfs_off_skips_gitattributes.md) — [test]
- [lfs_opt_in_writes_gitattributes](lfs_opt_in_writes_gitattributes.md) — [test]
- [open](open.md) — Open an existing library by `.snxlib` file path. The file's
- [open](open_1.md) — Open an existing library by `.snxlib` file path. The file's
- [recover_init](recover_init.md) — Recover a library whose `.git/` directory was deleted
- [recover_init](recover_init_1.md) — Recover a library whose `.git/` directory was deleted
- [rejects_non_snxlib_extension](rejects_non_snxlib_extension.md) — [test]
- [root](root.md) — Borrow the directory holding the `.snxlib` file (the git working tree).
- [root](root_1.md) — Borrow the directory holding the `.snxlib` file (the git working tree).
- [save_then_load_symbol_round_trip](save_then_load_symbol_round_trip.md) — [test]
- [validate_file_path](validate_file_path.md) — Reject paths whose extension isn't `.snxlib`. We refuse rather
- [validate_file_path](validate_file_path_1.md) — Reject paths whose extension isn't `.snxlib`. We refuse rather
