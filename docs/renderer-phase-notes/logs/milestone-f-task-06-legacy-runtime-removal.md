# Phase Note

## Metadata

- Phase: Milestone F (Execution)
- Task ID: 06
- Task name: remove remaining legacy runtime imports and source crate
- Owner: renderer-team
- Date: 2026-05-05
- Status: done

## Scope

Complete clean-room independence by removing all remaining app/ERC usage of the
legacy renderer crate, then delete the old `crates/oxide-render` source tree
from the workspace.

## Implementation summary

- Added local app runtime config module:
  - `crates/oxide-app/src/render_config.rs`
  - moved canvas font/style/grid/power-port/label/multisheet config APIs here
  - moved color conversion helper (`to_iced`) here
- Rewired `oxide-app` callsites from old crate namespace to local module:
  - app bootstrap/runtime/handlers/view/canvas/pcb path
  - preferences and font modules
  - regression test imports
- Removed `oxide-render` dependency from app and ERC manifests.
- Inlined local symbol transform logic in `oxide-erc` so ERC does not depend on
  any renderer crate.
- Removed `oxide-render` from root workspace members and shared dependencies.
- Deleted legacy crate source directory:
  - `crates/oxide-render/`

## Verification

Commands:

```text
cargo check
cargo test -p oxide-app --test regression --no-run
rg -n "\boxide_render::|\boxide-render\b" crates Cargo.toml crates/*/Cargo.toml
test ! -d crates/oxide-render && echo "deleted" || echo "still_exists"
```

Result:

- `cargo check`: pass
- `cargo test -p oxide-app --test regression --no-run`: pass
- Legacy renderer references in source/manifests: 0 matches
- Legacy source directory presence check: `deleted`

## Clean-room evidence

- Runtime-facing APIs required by app were implemented locally in app/erc crates.
- No legacy renderer source is left in workspace membership or source tree.
- Remaining renderer path is `oxide-renderer` only.

## Exit checklist

- [x] App has no legacy renderer crate imports
- [x] ERC has no legacy renderer crate dependency
- [x] Workspace has no legacy renderer crate member/dependency
- [x] Legacy source directory removed
- [x] Build/test compile validation completed
