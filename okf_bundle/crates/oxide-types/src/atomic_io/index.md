# atomic_io

## Functions

- [a_post_creation_failure_leaves_no_stray_tmp](a_post_creation_failure_leaves_no_stray_tmp.md) — [test]
- [atomic_write](atomic_write.md) — Atomically write `bytes` to `path`. Creates parent directories
- [creates_parent_directory](creates_parent_directory.md) — [test]
- [has_stray_tmp](has_stray_tmp.md) — True if `dir` contains a leftover atomic-write temp sibling
- [overwrites_existing_file](overwrites_existing_file.md) — [test]
- [round_trip_creates_destination](round_trip_creates_destination.md) — [test]
- [tmp_path_for](tmp_path_for.md) — Build a temp sibling path for `path`, unique per call: `<pid>-<counter>`
- [tmp_path_for_is_unique_per_call](tmp_path_for_is_unique_per_call.md) — [test]
- [writes_empty_and_large_payloads_and_leaves_no_tmp](writes_empty_and_large_payloads_and_leaves_no_tmp.md) — [test]
