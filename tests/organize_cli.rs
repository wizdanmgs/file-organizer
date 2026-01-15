use assert_cmd::cargo::cargo_bin_cmd;
use std::fs::File;
use tempfile::tempdir;

#[test]
fn cli_organizes_by_extension() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("file.txt")).unwrap();

    let mut cmd = cargo_bin_cmd!("nata");
    cmd.args([dir.path().to_str().unwrap(), "--by", "extension"])
        .assert()
        .success();

    assert!(dir.path().join("txt/file.txt").exists());
}
