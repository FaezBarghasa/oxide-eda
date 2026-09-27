---
okf_version: "0.2"
type: Function
title: identity_for_repo
description: Resolve the user identity for a commit. Walks a chain of sources
resource: crates/oxide-library/src/adapters/local_git/project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/src/adapters/local_git/project/identity_for_repo
language: rust
---

# identity_for_repo

Resolve the user identity for a commit. Walks a chain of sources

## Signature

```rust
fn identity_for_repo(repo: &git2::Repository) -> Result<(String, String), LibraryError>
```

## Docstring

Resolve the user identity for a commit. Walks a chain of sources
so the user's real name lands on the commit even when the OS
distribution didn't pre-populate `~/.gitconfig`:

1. Repo-local `user.name` / `user.email` (the standard git path).
2. Process env vars `GIT_AUTHOR_NAME` / `GIT_AUTHOR_EMAIL` and
`GIT_COMMITTER_NAME` / `GIT_COMMITTER_EMAIL` — git's own
fallback hierarchy, which Oxide honours so CI / scripted
flows don't need to mutate `~/.gitconfig`.
3. POSIX-style env hints `USER` / `EMAIL` for the convenience case
of "developer hasn't configured git yet but their shell knows
their identity". Email derives a synthetic local-host address
when `EMAIL` is unset (`<user>@<hostname>`).

Returns `Err(LibraryError::Backend("git identity not configured…"))`
when none of the above resolves a name. Durable project rule:
commits must carry the user's identity — never a generic
"Oxide Project" / "Oxide bot" fallback. Surfaces through the
async commit pipeline as a `Message::ProjectGitCommitDone` error
the user can act on.

## Source
Lines 483–531 in `crates/oxide-library/src/adapters/local_git/project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project](/crates/oxide-library/src/adapters/local_git/project.md) |
| called_by | [commit_path](/crates/oxide-library/src/adapters/local_git/project/commit_path.md) |
