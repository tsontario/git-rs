use std::fs;
use std::io::Write;
use std::path;

const DIRS: [&str; 5] = [
    ".git",
    ".git/refs/heads",
    ".git/refs/tags",
    ".git/objects/info",
    ".git/objects/pack",
];

const HEAD: &str = ".git/HEAD";

pub fn call(path: &path::Path) -> anyhow::Result<()> {
    let git_path = path.join(".git");
    match fs::exists(git_path) {
        Ok(true) => {
            eprintln!("Reinitializing git repository...")
        }
        Ok(false) => {
            eprintln!("Initializing git repository...")
        }
        Err(e) => {
            return Err(e.into());
        }
    }

    for subdir in DIRS.iter() {
        let dir = path.join(subdir);
        fs::create_dir_all(dir)?;
    }

    let head_path = path.join(HEAD);
    let mut file = fs::File::create(head_path)?;
    file.write_all(b"ref: refs/heads/main\n")?;

    Ok(())
}
