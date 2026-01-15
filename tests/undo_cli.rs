use assert_cmd::cargo::cargo_bin_cmd;
use std::fs::File;
use tempfile::tempdir;

#[test]
fn undo_restores_file_and_removes_created_dirs() {
    let dir = tempdir().unwrap();
    let base = dir.path();

    // Create files
    let txt = base.join("a.txt");
    let rs = base.join("b.rs");
    File::create(&txt).unwrap();
    File::create(&rs).unwrap();

    // Run organize
    let mut organize = cargo_bin_cmd!("nata");
    organize
        .args([base.to_str().unwrap(), "--by", "extension"])
        .assert()
        .success();

    // Verify files moved
    assert!(base.join("txt/a.txt").exists());
    assert!(base.join("rs/b.rs").exists());
    assert!(!txt.exists());
    assert!(!rs.exists());

    // Run undo
    let mut undo = cargo_bin_cmd!("nata");
    undo.args([base.to_str().unwrap(), "--undo"])
        .assert()
        .success();

    // Files restored
    assert!(txt.exists());
    assert!(rs.exists());

    // Created directories removed
    assert!(!base.join("txt").exists());
    assert!(!base.join("rs").exists());

    // Undo log removed
    assert!(!base.join(".kudu.log").exists());
}

#[test]
fn undo_without_log_fails_gracefully() {
    let dir = tempdir().unwrap();

    let mut undo = cargo_bin_cmd!("nata");
    undo.args([dir.path().to_str().unwrap(), "--undo"])
        .assert()
        .failure();
}

#[test]
fn dry_run_does_not_create_undo_log() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("a.txt")).unwrap();

    let mut cmd = cargo_bin_cmd!("nata");
    cmd.args([
        dir.path().to_str().unwrap(),
        "--by",
        "extension",
        "--dry-run",
    ])
    .assert()
    .success();

    assert!(!dir.path().join(".kudu.log").exists());
}
