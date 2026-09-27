---
okf_version: "0.2"
type: Function
title: drain_pending_git_commits
description: v0.23 — Drain the pending-commit queue. Returns a
resource: crates/oxide-app/src/app/handlers/document_files/git.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/git/drain_pending_git_commits
language: rust
---

# drain_pending_git_commits

v0.23 — Drain the pending-commit queue. Returns a

## Signature

```rust
impl Oxide { pub(crate) fn drain_pending_git_commits(&mut self) -> iced::Task<crate::app::Message> }
```

## Visibility

- `pub(crate)`

## Docstring

v0.23 — Drain the pending-commit queue. Returns a
`Task::batch` of `Task::perform` calls that each open the
project's git adapter and run `commit_path` on a tokio
`spawn_blocking`. Each completion routes through
`Message::Project(ProjectMsg::GitCommitDone)` which clears the
matching `inflight_git_commits` entry. Returns `Task::none()` when the
queue is empty.

**Ordering note:** Concurrent commits to the same project
repo are serialised by libgit2's `.git/index.lock` (and the
`LocalGitProjectAdapter::git_lock` mutex), but **not** by
this dispatcher. If the user types fast enough to fire two
saves before the first commit completes, the second
`Task::perform` may race the first; in practice both
commits land sequentially with the OS-level lock determining
order. The first commit's blob can therefore reflect
post-second-save content if the rapid sequence overlaps
`index.add_path` with the user's next save. Not data loss —
every save's content is captured by *some* commit — but the
commit-message vs blob-content correspondence is best-effort.

## Source
Lines 89–136 in `crates/oxide-app/src/app/handlers/document_files/git.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [git](/crates/oxide-app/src/app/handlers/document_files/git.md) |
