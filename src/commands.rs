pub mod cat_file;
pub mod hash_object;
pub mod init;

pub struct CliConfig {
    pub work_dir: String,
    pub git_dir: Option<std::path::PathBuf>,
}

impl CliConfig {
    pub fn build(work_dir: String) -> CliConfig {
        CliConfig {
            work_dir: work_dir.clone(),
            git_dir: Self::resolve_git_dir(std::path::PathBuf::from(work_dir.clone())),
        }
    }

    fn resolve_git_dir(path: std::path::PathBuf) -> Option<std::path::PathBuf> {
        let mut dir = std::fs::canonicalize(std::path::PathBuf::from(path)).unwrap();
        loop {
            let git_dir = dir.join(".git");
            if git_dir.exists() {
                return Some(git_dir);
            }
            if dir.parent().is_none() {
                return None;
            }
            dir = dir.parent().unwrap().to_path_buf();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn resolve_git_dir_finds_git_in_work_dir() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".git")).unwrap();

        let config = CliConfig::build(tmp.path().to_str().unwrap().to_string());
        assert_eq!(config.git_dir, Some(tmp.path().join(".git")));
    }

    #[test]
    fn resolve_git_dir_finds_git_in_ancestor() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".git")).unwrap();
        let child = tmp.path().join("sub").join("deep");
        fs::create_dir_all(&child).unwrap();

        let config = CliConfig::build(child.to_str().unwrap().to_string());
        assert_eq!(config.git_dir, Some(tmp.path().join(".git")));
    }

    #[test]
    fn resolve_git_dir_from_within_git_dir() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join(".git").join("objects")).unwrap();
        let child = tmp.path().join(".git").join("objects");

        let config = CliConfig::build(child.to_str().unwrap().to_string());
        assert_eq!(config.git_dir, Some(tmp.path().join(".git")));
    }
}
