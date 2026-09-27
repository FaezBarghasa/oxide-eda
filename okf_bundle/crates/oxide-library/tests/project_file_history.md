---
okf_version: "0.2"
type: Module
title: project_file_history
description: "Integration tests for `oxide_library::project_file_history`."
resource: crates/oxide-library/tests/project_file_history.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/tests/project_file_history
language: rust
---

# project_file_history

Integration tests for `oxide_library::project_file_history`.

## Docstring

Integration tests for `oxide_library::project_file_history`.

Mirrors the in-adapter `LocalGitAdapter::history` tests but
exercises the public helper used by `oxide-app`'s right-dock
History panel. The helper walks any git repo (not just a
library-rooted one), so the fixtures here build a plain
`git2::Repository` and stage handful of commits manually.

## Relationships

| Type | Target |
|------|--------|
| related | [fixture_signature](/crates/oxide-library/tests/project_file_history/fixture_signature.md) |
| related | [commit_file](/crates/oxide-library/tests/project_file_history/commit_file.md) |
| related | [init_repo](/crates/oxide-library/tests/project_file_history/init_repo.md) |
| related | [returns_not_found_when_no_dot_git](/crates/oxide-library/tests/project_file_history/returns_not_found_when_no_dot_git.md) |
| related | [returns_empty_for_path_with_no_commits](/crates/oxide-library/tests/project_file_history/returns_empty_for_path_with_no_commits.md) |
| related | [returns_empty_on_unborn_head](/crates/oxide-library/tests/project_file_history/returns_empty_on_unborn_head.md) |
| related | [returns_n_commits_newest_first](/crates/oxide-library/tests/project_file_history/returns_n_commits_newest_first.md) |
| related | [accepts_absolute_path_under_project_dir](/crates/oxide-library/tests/project_file_history/accepts_absolute_path_under_project_dir.md) |
