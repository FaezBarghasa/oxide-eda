# project

## Classs

- [CommitPathStats](CommitPathStats.md) — Diff-stat summary for one commit's touch on a single file.
- [LocalGitProjectAdapter](LocalGitProjectAdapter.md) — Project-scoped git adapter.

## Functions

- [commit_diff_stats_for_path](commit_diff_stats_for_path.md) — Returns `Some(stats)` when `commit` touched `rel_str`; `None` when
- [commit_external_change](commit_external_change.md) — Commit an externally-edited file (user opened the file in a
- [commit_external_change](commit_external_change_1.md) — Commit an externally-edited file (user opened the file in a
- [commit_path](commit_path.md) — Commit a single file with the supplied message. `rel_path` is
- [commit_path](commit_path_1.md) — Commit a single file with the supplied message. `rel_path` is
- [file_history](file_history.md) — Per-file commit history newest-first. Returns up to `limit`
- [file_history](file_history_1.md) — Per-file commit history newest-first. Returns up to `limit`
- [history_entry_from_commit](history_entry_from_commit.md)
- [identity_for_repo](identity_for_repo.md) — Resolve the user identity for a commit. Walks a chain of sources
- [open_or_init](open_or_init.md) — Open the git repo at `project_root`, or `git init` if no
- [open_or_init](open_or_init_1.md) — Open the git repo at `project_root`, or `git init` if no
- [project_root](project_root.md) — Project root that this adapter manages.
- [project_root](project_root_1.md) — Project root that this adapter manages.
- [restore_at](restore_at.md) — Restore `rel_path` to the state captured by `commit_oid`.
- [restore_at](restore_at_1.md) — Restore `rel_path` to the state captured by `commit_oid`.
- [restore_at_from_sha](restore_at_from_sha.md) — String-SHA-keyed alternative to [`restore_at`]. Parses the
- [restore_at_from_sha](restore_at_from_sha_1.md) — String-SHA-keyed alternative to [`restore_at`]. Parses the
- [write_gitattributes](write_gitattributes.md) — Write the project's `.gitattributes` file with the v0.22 spec:
- [write_gitattributes](write_gitattributes_1.md) — Write the project's `.gitattributes` file with the v0.22 spec:
