# Phase Note

## Metadata

- Phase: Milestone F (Execution)
- Task ID: 02
- Task name: app compatibility bridge for schematic runtime contracts
- Owner: renderer-team
- Date: 2026-05-05
- Status: done

## Scope

Introduce a schematic runtime bridge module in `oxide-app` and migrate all
app source modules to use that bridge instead of directly calling
`oxide_render::schematic`.

## Implementation summary

- Added bridge module:
  - `crates/oxide-app/src/schematic_runtime.rs`
- Exposed bridge at crate root:
  - `crates/oxide-app/src/lib.rs` (`pub mod schematic_runtime;`)
- Migrated direct callsites:
  - Replaced `oxide_render::schematic::...` with
    `crate::schematic_runtime::...` across app modules.

Post-migration verification command:

```text
rg -n "oxide_render::schematic::" crates/oxide-app/src
```

Result:

- Remaining direct references: only inside the bridge module.
- No direct `oxide_render::schematic` references in other app source modules.

Migrated module set (13 files):

- `crates/oxide-app/src/canvas/mod.rs`
- `crates/oxide-app/src/panels/mod.rs`
- `crates/oxide-app/src/app/bootstrap.rs`
- `crates/oxide-app/src/app/state.rs`
- `crates/oxide-app/src/app/load_gateway.rs`
- `crates/oxide-app/src/app/mutation_gateway.rs`
- `crates/oxide-app/src/app/dispatch/mod.rs`
- `crates/oxide-app/src/app/dispatch/text_edit.rs`
- `crates/oxide-app/src/app/handlers/canvas.rs`
- `crates/oxide-app/src/app/handlers/erc.rs`
- `crates/oxide-app/src/app/handlers/selection_workflow.rs`
- `crates/oxide-app/src/app/handlers/dock/property_editor.rs`
- `crates/oxide-app/src/app/view/dialogs.rs`

## Cutover impact

- Direct legacy runtime usage is now centralized and controllable via one module.
- Future Task 03/04 swaps can target bridge internals first, minimizing
  multi-file churn and regression risk.

## Clean-room evidence

- Source: Milestone F Task 02 scope from issue/checklist.
- Derivation: centralized bridge pattern used to isolate legacy runtime surface.
- Rationale: prepare low-risk incremental replacement of bridge internals with
  `oxide-renderer` APIs while preserving app behavior.
- Clean-room check: No GPL-licensed source consulted.
- Verification: callsite scan confirms only bridge module retains direct
  `oxide_render::schematic` imports.

## Artifacts

- PR/commit: pending
- Test output: pending (covered in Task 07 validation pass)
- Screenshot/benchmark: n/a

## Exit checklist

- [x] Implementation completed
- [x] Source and derivation documented
- [x] Clean-room check confirmed
- [x] Verification artifact added
- [x] Linked from issue/checklist
