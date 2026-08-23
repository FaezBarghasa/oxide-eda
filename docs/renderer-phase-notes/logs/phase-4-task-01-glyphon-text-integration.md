# Phase Note

## Metadata

- Phase: 4
- Task ID: 01
- Task name: glyphon text pipeline integration
- Owner: renderer-team
- Date: 2026-05-04
- Status: done

## Scope

Integrate glyphon-backed text rendering into the renderer text pipeline.

## Implementation notes

- Added `GlyphonTextPipeline` as the only text rendering path in oxide-gfx.
- Removed the legacy placeholder quad text pipeline and deleted the obsolete `shader/text.wgsl` path.
- Updated text smoke pass integration to run through glyphon prepare/render and atlas trim lifecycle.
- Updated wgpu descriptor usage for smoke paths and primitive pipelines to keep text validation passing on the active dependency line.

## Clean-room evidence

- Source: glyphon public docs and wgpu public docs.
- Derivation: direct adapter integration from scene text items to glyphon buffer updates and text-area submission.
- Rationale: replace placeholder text quads with production glyph shaping/rasterization and remove fallback behavior.
- Clean-room check: No GPL-licensed source consulted
- Verification: `cargo test -p oxide-gfx -- --nocapture`, `cargo test -p oxide-renderer -- --nocapture`, `cargo check -p oxide-gfx -p oxide-renderer`, and `cargo build -p oxide-gfx -p oxide-renderer` passed.

## Artifacts

- PR/commit: pending
- Test output: oxide-gfx (15 passed), oxide-renderer (4 passed)
- Screenshot/benchmark: pending

## Exit checklist

- [x] Implementation completed
- [x] Source and derivation documented
- [x] Clean-room check confirmed
- [x] Verification artifact added
- [x] Linked from issue/checklist
