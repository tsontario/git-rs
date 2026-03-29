use std::io::Write;
use tempfile::TempDir;

/// Initializes a bare repo in a temporary directory and creates a single file.
///
/// Returns both the root directory and created tempfile.
pub fn init_simple_git_dir() -> anyhow::Result<(TempDir, tempfile::NamedTempFile, std::path::PathBuf)> {
    let tempdir = tempfile::tempdir().unwrap();
    let mut tempfile = tempfile::Builder::new().tempfile_in(tempdir.path())?;
    tempfile.write_all(b"hello world")?;
    my_git::commands::init::call(tempdir.path())?;
    let git_dir = tempdir.path().join(".git");

    Ok((tempdir, tempfile, git_dir))
}