use std::path::{Path, PathBuf};
use tempfile::TempDir;

use oxide_library::dependency::{GitDependencyManager, LockfileManager};
use oxide_types::project::{
    DependencyKind, GitReference, GitSource, ProjectData, ProjectDependency,
};

/// Helper: create a git repository with sample commits and semantic version tags.
fn create_test_git_repo() -> (TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_path = dir.path().to_path_buf();
    let repo = git2::Repository::init(&repo_path).expect("git init");

    let sig = git2::Signature::now("Oxide Test", "test@oxide.local").expect("signature");

    // Commit 1 (v0.1.0)
    let file1 = repo_path.join("component.snxsym");
    std::fs::write(&file1, b"symbol: STM32F407").expect("write file1");

    let mut index = repo.index().expect("index");
    index.add_path(Path::new("component.snxsym")).expect("add path");
    index.write().expect("index write");
    let tree_id = index.write_tree().expect("write tree");
    let tree = repo.find_tree(tree_id).expect("find tree");

    let commit1_id = repo
        .commit(Some("HEAD"), &sig, &sig, "v0.1.0 release", &tree, &[])
        .expect("commit 1");
    let commit1 = repo.find_commit(commit1_id).expect("find commit1");
    repo.tag("v0.1.0", &commit1.as_object(), &sig, "v0.1.0 tag", false)
        .expect("tag v0.1.0");

    // Commit 2 (v1.0.0)
    let file2 = repo_path.join("library.toml");
    std::fs::write(&file2, b"schema = 1\nname = \"connectors\"").expect("write file2");
    index.add_path(Path::new("library.toml")).expect("add path 2");
    index.write().expect("index write 2");
    let tree2_id = index.write_tree().expect("write tree 2");
    let tree2 = repo.find_tree(tree2_id).expect("find tree 2");

    let commit2_id = repo
        .commit(
            Some("HEAD"),
            &sig,
            &sig,
            "v1.0.0 release",
            &tree2,
            &[&commit1],
        )
        .expect("commit 2");
    let commit2 = repo.find_commit(commit2_id).expect("find commit2");
    repo.tag("v1.0.0", &commit2.as_object(), &sig, "v1.0.0 tag", false)
        .expect("tag v1.0.0");

    // Commit 3 (v1.1.0)
    let file3 = repo_path.join("footprint.snxfpt");
    std::fs::write(&file3, b"footprint: LQFP-100").expect("write file3");
    index.add_path(Path::new("footprint.snxfpt")).expect("add path 3");
    index.write().expect("index write 3");
    let tree3_id = index.write_tree().expect("write tree 3");
    let tree3 = repo.find_tree(tree3_id).expect("find tree 3");

    let commit3_id = repo
        .commit(
            Some("HEAD"),
            &sig,
            &sig,
            "v1.1.0 release",
            &tree3,
            &[&commit2],
        )
        .expect("commit 3");
    let commit3 = repo.find_commit(commit3_id).expect("find commit3");
    repo.tag("v1.1.0", &commit3.as_object(), &sig, "v1.1.0 tag", false)
        .expect("tag v1.1.0");

    // Commit 4 (v2.0.0 - breaking)
    let file4 = repo_path.join("v2_breaking.txt");
    std::fs::write(&file4, b"v2 breaking change").expect("write file4");
    index.add_path(Path::new("v2_breaking.txt")).expect("add path 4");
    index.write().expect("index write 4");
    let tree4_id = index.write_tree().expect("write tree 4");
    let tree4 = repo.find_tree(tree4_id).expect("find tree 4");

    let commit4_id = repo
        .commit(
            Some("HEAD"),
            &sig,
            &sig,
            "v2.0.0 release",
            &tree4,
            &[&commit3],
        )
        .expect("commit 4");
    let commit4 = repo.find_commit(commit4_id).expect("find commit4");
    repo.tag("v2.0.0", &commit4.as_object(), &sig, "v2.0.0 tag", false)
        .expect("tag v2.0.0");

    (dir, repo_path)
}

#[test]
fn test_dependency_resolution_semver() {
    let (_remote_guard, remote_path) = create_test_git_repo();
    let project_dir = tempfile::tempdir().expect("project dir");
    let project_root = project_dir.path();

    let mut project_data = ProjectData {
        name: "TestProject".to_string(),
        dir: project_root.to_string_lossy().to_string(),
        schematic_root: None,
        pcb_file: None,
        sheets: vec![],
        variant_definitions: vec![],
        active_variant: None,
        libraries: vec![],
        dependencies: vec![ProjectDependency {
            name: "connectors".to_string(),
            kind: DependencyKind::Library,
            source: GitSource {
                url: remote_path.to_string_lossy().to_string(),
                reference: GitReference::Semver("^1.0.0".to_string()),
            },
            subpath: None,
            enabled: true,
        }],
        enable_git: false,
    };

    let manager = GitDependencyManager::new();
    let lockfile = manager
        .resolve_and_install(project_root, &project_data)
        .expect("resolve and install");

    assert_eq!(lockfile.version, 1);
    let locked = lockfile.dependencies.get("connectors").expect("locked entry");
    assert_eq!(locked.resolved_version.as_deref(), Some("1.1.0"));
    assert_eq!(locked.kind, DependencyKind::Library);

    // Verify files checked out
    let installed_file = project_root
        .join(".oxide")
        .join("deps")
        .join("connectors")
        .join("footprint.snxfpt");
    assert!(installed_file.exists());

    // Verify v2_breaking is NOT present (since constraint is ^1.0.0)
    let v2_file = project_root
        .join(".oxide")
        .join("deps")
        .join("connectors")
        .join("v2_breaking.txt");
    assert!(!v2_file.exists());

    // Verify project.lock written to disk
    let disk_lock = LockfileManager::read_lockfile(project_root)
        .expect("read lockfile")
        .expect("lockfile exists");
    assert_eq!(disk_lock, lockfile);

    // Verify integrity check
    let report = manager
        .verify_integrity(project_root, &lockfile)
        .expect("verify integrity");
    assert!(report.is_clean());
    assert_eq!(report.valid_dependencies, vec!["connectors"]);

    // Verify mounting into ProjectData
    let mount_report = manager
        .mount_dependencies(project_root, &lockfile, &mut project_data)
        .expect("mount");
    assert_eq!(mount_report.mounted_libraries.len(), 1);
    assert_eq!(project_data.libraries.len(), 1);
    assert_eq!(
        project_data.libraries[0].path,
        PathBuf::from(".oxide/deps/connectors")
    );
}

#[test]
fn test_dependency_exact_tag_and_restore() {
    let (_remote_guard, remote_path) = create_test_git_repo();
    let project_dir = tempfile::tempdir().expect("project dir");
    let project_root = project_dir.path();

    let project_data = ProjectData {
        name: "TestProject".to_string(),
        dir: project_root.to_string_lossy().to_string(),
        schematic_root: None,
        pcb_file: None,
        sheets: vec![],
        variant_definitions: vec![],
        active_variant: None,
        libraries: vec![],
        dependencies: vec![ProjectDependency {
            name: "mcu".to_string(),
            kind: DependencyKind::Component,
            source: GitSource {
                url: remote_path.to_string_lossy().to_string(),
                reference: GitReference::Tag("v0.1.0".to_string()),
            },
            subpath: None,
            enabled: true,
        }],
        enable_git: false,
    };

    let manager = GitDependencyManager::new();
    let lockfile = manager
        .resolve_and_install(project_root, &project_data)
        .expect("resolve");

    let locked = lockfile.dependencies.get("mcu").expect("mcu locked");
    assert_eq!(locked.resolved_version.as_deref(), Some("0.1.0"));
    assert_eq!(locked.kind, DependencyKind::Component);

    // Delete installed cache directory
    let cache_dir = project_root.join(".oxide").join("deps").join("mcu");
    std::fs::remove_dir_all(&cache_dir).expect("remove cache");

    let verify_missing = manager
        .verify_integrity(project_root, &lockfile)
        .expect("verify missing");
    assert_eq!(verify_missing.missing_dependencies, vec!["mcu"]);

    // Restore from lockfile
    manager
        .restore_from_lockfile(project_root, &lockfile)
        .expect("restore");
    let verify_restored = manager
        .verify_integrity(project_root, &lockfile)
        .expect("verify restored");
    assert!(verify_restored.is_clean());
}
