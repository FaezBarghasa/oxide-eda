//! Git revision loader for visual schematic and PCB diff-merge.
//!
//! Extracts `.snxpcb` design files directly from Git commit trees and blobs,
//! enabling zero-cloud, local-first visual diffs between arbitrary revisions
//! (`HEAD`, `HEAD~1`, branches, or working tree).

use anyhow::{Context, Result, anyhow};
use git2::{ErrorCode, Repository};
use oxide_types::format::SnxPcb;
use oxide_types::pcb::PcbBoard;
use std::path::Path;

use super::PcbVisualDiff;

/// Extracts and deserializes a `PcbBoard` from a Git revision or working tree.
///
/// # Arguments
/// * `repo_path` - Path to the git repository or project directory containing `.git`.
/// * `rev` - Git revision identifier (`"HEAD"`, `"HEAD~1"`, commit hash, branch name, or `"WORKING"`/`"WORKTREE"`).
/// * `file_rel_path` - Relative path of the `.snxpcb` file from repo root.
pub fn load_board_revision(repo_path: &Path, rev: &str, file_rel_path: &str) -> Result<PcbBoard> {
    if rev.eq_ignore_ascii_case("WORKING") || rev.eq_ignore_ascii_case("WORKTREE") || rev.is_empty()
    {
        let abs_path = repo_path.join(file_rel_path);
        let content = std::fs::read_to_string(&abs_path)
            .with_context(|| format!("Failed to read working tree file: {:?}", abs_path))?;
        let snx = SnxPcb::parse(&content)
            .map_err(|e| anyhow!("Failed to parse .snxpcb at {:?}: {}", abs_path, e))?;
        return Ok(snx.board);
    }

    let repo = Repository::discover(repo_path)
        .with_context(|| format!("Failed to open git repository at {:?}", repo_path))?;

    let obj = repo
        .revparse_single(rev)
        .with_context(|| format!("Failed to resolve git revision '{}'", rev))?;

    let tree = obj
        .peel_to_tree()
        .with_context(|| format!("Failed to peel revision '{}' to git tree", rev))?;

    let normalized_path = file_rel_path.trim_start_matches(['/', '\\']);
    let entry = tree
        .get_path(Path::new(normalized_path))
        .with_context(|| format!("File '{}' not found in revision '{}'", normalized_path, rev))?;

    let blob = repo
        .find_blob(entry.id())
        .with_context(|| format!("Failed to load blob for '{}' at '{}'", normalized_path, rev))?;

    let content = std::str::from_utf8(blob.content()).with_context(|| {
        format!(
            "File '{}' in revision '{}' contains invalid UTF-8 bytes",
            normalized_path, rev
        )
    })?;

    let snx = SnxPcb::parse(content).map_err(|e| {
        anyhow!(
            "Failed to parse .snxpcb blob '{}' at revision '{}': {}",
            normalized_path,
            rev,
            e
        )
    })?;

    Ok(snx.board)
}

/// Computes the visual diff between two Git revisions of a PCB layout.
pub fn load_diff_between_revisions(
    repo_path: &Path,
    rev_a: &str,
    rev_b: &str,
    file_rel_path: &str,
) -> Result<PcbVisualDiff> {
    let board_a = load_board_revision(repo_path, rev_a, file_rel_path)
        .with_context(|| format!("Failed to load baseline revision '{}'", rev_a))?;
    let board_b = load_board_revision(repo_path, rev_b, file_rel_path)
        .with_context(|| format!("Failed to load comparison revision '{}'", rev_b))?;

    Ok(PcbVisualDiff::compute_diff(&board_a, &board_b))
}

/// Lists recent commit revisions (short hash and summary) from HEAD.
pub fn list_recent_revisions(repo_path: &Path, max_count: usize) -> Result<Vec<(String, String)>> {
    let repo = match Repository::discover(repo_path) {
        Ok(r) => r,
        Err(e) if e.code() == ErrorCode::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e.into()),
    };

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let mut commits = Vec::new();
    for id_res in revwalk.take(max_count) {
        let oid = id_res?;
        if let Ok(commit) = repo.find_commit(oid) {
            let short = oid.to_string()[..7.min(oid.to_string().len())].to_string();
            let raw = commit.message().unwrap_or("");
            let summary = raw.lines().next().unwrap_or("").to_string();
            commits.push((short, summary));
        }
    }

    Ok(commits)
}

/// Asynchronously loads a visual diff between two revisions off the UI thread.
pub fn load_diff_async(
    repo_path: std::path::PathBuf,
    base_rev: String,
    target_rev: String,
    file_rel_path: String,
) -> iced::Task<crate::app::Message> {
    iced::Task::perform(
        async move {
            tokio::task::spawn_blocking(move || {
                load_diff_between_revisions(&repo_path, &base_rev, &target_rev, &file_rel_path)
            })
            .await
            .unwrap_or_else(|e| Err(anyhow!("spawn_blocking error: {e}")))
        },
        |result| match result {
            Ok(diff) => crate::app::Message::Dock(crate::dock::DockMessage::Panel(
                crate::panels::PanelMsg::DiffLoaded(Box::new(diff)),
            )),
            Err(err) => {
                crate::diagnostics::log_warning(format!("Failed to load git diff: {err}"));
                crate::app::Message::Noop
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_types::pcb::{Footprint, Point};
    use tempfile::TempDir;

    #[test]
    fn test_git_loader_blob_extraction_and_diff() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let repo_path = temp_dir.path();

        // 1. Initialize git repo
        let repo = Repository::init(repo_path).expect("git init");
        let sig = git2::Signature::now("Oxide Test", "test@oxide.local").expect("signature");

        // 2. Commit initial board revision (rev_a)
        let mut board_v1 = PcbBoard::default();
        let r1 = Footprint {
            uuid: uuid::Uuid::new_v4(),
            reference: "R1".to_string(),
            value: "10k".to_string(),
            footprint_id: "R_0603".to_string(),
            position: Point { x: 10.0, y: 10.0 },
            rotation: 0.0,
            layer: "F.Cu".to_string(),
            locked: false,
            pads: Vec::new(),
            graphics: Vec::new(),
            properties: Vec::new(),
        };
        board_v1.footprints.push(r1);

        let snx_v1 = SnxPcb::new(board_v1.clone());
        let file_path = repo_path.join("board.snxpcb");
        std::fs::write(&file_path, snx_v1.write_string().expect("write snx")).expect("write file");

        let mut index = repo.index().expect("index");
        index.add_path(Path::new("board.snxpcb")).expect("add");
        index.write().expect("write index");
        let tree_id = index.write_tree().expect("write tree");
        let tree = repo.find_tree(tree_id).expect("find tree");
        let commit_v1_id = repo
            .commit(Some("HEAD"), &sig, &sig, "Initial commit v1", &tree, &[])
            .expect("commit v1");

        // 3. Modify board and commit rev_b
        let mut board_v2 = board_v1.clone();
        board_v2.footprints[0].value = "20k".to_string();
        let c1 = Footprint {
            uuid: uuid::Uuid::new_v4(),
            reference: "C1".to_string(),
            value: "100nF".to_string(),
            footprint_id: "C_0603".to_string(),
            position: Point { x: 25.0, y: 15.0 },
            rotation: 0.0,
            layer: "F.Cu".to_string(),
            locked: false,
            pads: Vec::new(),
            graphics: Vec::new(),
            properties: Vec::new(),
        };
        board_v2.footprints.push(c1);

        let snx_v2 = SnxPcb::new(board_v2);
        std::fs::write(&file_path, snx_v2.write_string().expect("write snx")).expect("write file");

        index.add_path(Path::new("board.snxpcb")).expect("add v2");
        index.write().expect("write index");
        let tree2_id = index.write_tree().expect("write tree 2");
        let tree2 = repo.find_tree(tree2_id).expect("find tree 2");
        let parent = repo.find_commit(commit_v1_id).expect("parent commit");
        let commit_v2_id = repo
            .commit(
                Some("HEAD"),
                &sig,
                &sig,
                "Update R1 value and add C1",
                &tree2,
                &[&parent],
            )
            .expect("commit v2");

        // 4. Test loading revisions via git_loader
        let v1_str = commit_v1_id.to_string();
        let v2_str = commit_v2_id.to_string();

        let loaded_v1 =
            load_board_revision(repo_path, &v1_str, "board.snxpcb").expect("load revision v1");
        assert_eq!(loaded_v1.footprints.len(), 1);
        assert_eq!(loaded_v1.footprints[0].value, "10k");

        let loaded_v2 =
            load_board_revision(repo_path, &v2_str, "board.snxpcb").expect("load revision v2");
        assert_eq!(loaded_v2.footprints.len(), 2);
        assert_eq!(loaded_v2.footprints[0].value, "20k");

        // 5. Test computing diff between git revisions
        let diff = load_diff_between_revisions(repo_path, &v1_str, &v2_str, "board.snxpcb")
            .expect("diff revisions");
        assert_eq!(diff.added_count, 1, "C1 added");
        assert_eq!(diff.modified_count, 1, "R1 modified");
        assert_eq!(diff.removed_count, 0, "No removed components");

        // 6. Test list_recent_revisions
        let recent = list_recent_revisions(repo_path, 5).expect("recent commits");
        assert_eq!(recent.len(), 2);
        assert!(recent[0].1.contains("Update R1 value"));
        assert!(recent[1].1.contains("Initial commit v1"));

        // 7. Test async git diff task constructor
        let _task = load_diff_async(
            repo_path.to_path_buf(),
            v1_str,
            v2_str,
            "board.snxpcb".to_string(),
        );
    }
}
