# Phase Note

## Metadata

- Phase: 6
- Task ID: 03
- Task name: theme dirty path without geometry rebuild
- Owner: renderer-team
- Date: 2026-05-05
- Status: done

## Scope

Implement `THEME` dirty behavior so palette updates avoid geometry rebuild.

## Implementation notes

- Added `UploadCounters::geometry_uploads` and `UploadCounters::is_theme_only_refresh` helpers to make theme-only behavior explicit and testable.
- Kept `DirtyFlags::THEME` path isolated to `refresh_theme` in upload gating.
- Added focused tests for `THEME`-only dirty transitions to confirm no geometry upload calls are triggered.
- Added mixed dirty-path test (`THEME | LINES`) to verify theme refresh coexists with geometry updates when explicitly requested.

## Clean-room evidence

- Source: theme policy in renderer plan.
- Derivation: uniform-only update path gated by `DirtyFlags::THEME`.
- Rationale: achieve instant theme switching with minimal GPU work.
- Clean-room check: No GPL-licensed source consulted
- Verification: `cargo test -p oxide-gfx scene::upload -- --nocapture`, `cargo test -p oxide-gfx -- --nocapture`, `cargo test -p oxide-renderer -- --nocapture`, `cargo check -p oxide-gfx -p oxide-renderer`, and `cargo build -p oxide-gfx -p oxide-renderer` passed.

## Artifacts

- PR/commit: pending
- Test output: oxide-gfx tests passed (39 passed, 0 failed), oxide-renderer tests passed (9 passed, 0 failed)
- Screenshot/benchmark: pending

## Exit checklist

- [x] Implementation completed
- [x] Source and derivation documented
- [x] Clean-room check confirmed
- [x] Verification artifact added
- [x] Linked from issue/checklist
