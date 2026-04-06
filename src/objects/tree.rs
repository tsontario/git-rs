use crate::objects::object::ObjectType;
use crate::objects::utils;
use std::fmt::{Display, Formatter};
use std::path;

pub struct Tree {
    pub entries: Vec<TreeEntry>,
}

#[derive(Clone)]
pub struct TreeEntry {
    pub mode: u32,
    pub filename: String,
    pub hash: String,
    pub size: usize,
}

pub struct TreeParser {
    /// The bytes to parse.
    bytes: Vec<u8>,
    /// optional path prefix to prepend to each entry's filename.
    path_prefix: Option<path::PathBuf>,
}

impl Tree {
    /// Outputs the byte representation of the tree. Suitable for converting to a format that is
    /// stored in the object store.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        for entry in self.entries.iter() {
            result.extend(format!("{} {}\0", entry.mode, entry.filename).as_bytes());
            result.extend(utils::string_to_bytes(&entry.hash));
        }

        result
    }
}

impl TreeParser {
    /// Creates a new TreeParser
    ///
    /// # Arguments
    /// * `bytes` - The raw bytes to parse
    /// * `path_prefix` - If provided, prefixes all filenames with this path
    pub fn new(bytes: &[u8], path_prefix: Option<path::PathBuf>) -> Self {
        TreeParser {
            bytes: bytes.to_vec(),
            path_prefix,
        }
    }

    /// Parses a slice of bytes into a vector of tree entries.
    pub fn parse(&self) -> anyhow::Result<Vec<TreeEntry>> {
        let mut offset: usize = 0;
        let mut entries: Vec<TreeEntry> = Vec::new();
        while offset < self.bytes.len() {
            let (entry, new_offset) = self.parse_one(&self.bytes[offset..])?;
            entries.push(entry);
            offset += new_offset;
        }
        Ok(entries)
    }

    /// Parses a single tree entry from a slice of bytes.
    /// Returns the parsed entry and the offset of the (the possible) next entry.
    /// It is the caller's responsibility to check if the offset is valid.
    fn parse_one(&self, bytes: &[u8]) -> anyhow::Result<(TreeEntry, usize)> {
        let null_pos = bytes.iter().position(|&b| b == 0).unwrap();
        let space_pos = bytes.iter().position(|&b| b == b' ').unwrap();

        let mode = std::str::from_utf8(&bytes[..space_pos])
            .unwrap()
            .parse::<u32>()?;
        let filename = String::from_utf8(bytes[space_pos + 1..null_pos].to_vec())?;
        let expanded_filename = self.expand_filename(&filename);
        let hash = utils::bytes_to_string(&bytes[null_pos + 1..null_pos + 21]);
        let size = null_pos + 21;

        // null_pos + 22 is the offset of the next entry
        Ok((
            TreeEntry {
                mode,
                hash,
                size,
                filename: expanded_filename,
            },
            size,
        ))
    }

    /// Prefixes the entry with path_prefix, if path_prefix is not None.
    fn expand_filename(&self, filename: &str) -> String {
        match self.path_prefix {
            Some(ref path_prefix) => path_prefix.join(filename).display().to_string(),
            None => String::from(filename),
        }
    }
}

pub enum Mode {
    RegularFile,
    ExecutableFile,
    Directory,
    SymLink,
    GitLink,
}

impl Mode {
    pub fn from_u32(mode: u32) -> anyhow::Result<Self> {
        match mode {
            100644 => Ok(Mode::RegularFile),
            100755 => Ok(Mode::ExecutableFile),
            40000 => Ok(Mode::Directory),
            120000 => Ok(Mode::SymLink),
            160000 => Ok(Mode::GitLink),
            _ => Err(anyhow::anyhow!("unknown mode: {}", mode)),
        }
    }

    pub fn object_type(&self) -> ObjectType {
        match self {
            Mode::RegularFile | Mode::ExecutableFile | Mode::SymLink => ObjectType::Blob,
            Mode::Directory => ObjectType::Tree,
            Mode::GitLink => ObjectType::Commit,
        }
    }
}

impl TreeEntry {
    pub(crate) fn object_type(&self) -> ObjectType {
        Mode::from_u32(self.mode).unwrap().object_type()
    }
}

impl Display for TreeEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:06} {} {}\t{}",
            self.mode,
            self.object_type(),
            self.hash,
            self.filename
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::utils::bytes_to_string;
    use sha1::Digest;

    #[test]
    fn parse_single_tree_entry() {
        let hash = random_hash(None);
        let mut bytes = b"100644 file1.txt\0".to_vec();
        bytes.extend_from_slice(&hash);
        let parser = TreeParser::new(&bytes, None);
        let entries = parser.parse().unwrap();
        assert_eq!(entries.len(), 1);
        let entry = entries.get(0).unwrap();
        assert_eq!(entry.mode, 100644);
        assert_eq!(entry.filename, "file1.txt");
        assert_eq!(entry.hash, bytes_to_string(&hash));
        assert_eq!(entry.size, 37);
    }

    #[test]
    fn parse_multiple_tree_entry() {
        let entries = build_tree_lines(3);
        let parser = TreeParser::new(&entries, None);
        let parsed_entries = parser.parse().unwrap();
        assert_eq!(parsed_entries.len(), 3);
        for i in 0..3 {
            let entry = parsed_entries.get(i).unwrap();
            let expected_hash = random_hash(Some(format!("{}", i)));
            assert_eq!(entry.mode, 100644);
            assert_eq!(entry.filename, format!("file{}.txt", i));
            assert_eq!(entry.hash, bytes_to_string(&expected_hash));
            assert_eq!(entry.size, 37);
        }
    }

    fn build_tree_lines(len: usize) -> Vec<u8> {
        let mut lines = Vec::new();
        for i in 0..len {
            let hash = random_hash(Some(format!("{}", i)));
            let mut line = format!("100644 file{}.txt\0", i).as_bytes().to_vec();
            line.extend_from_slice(&hash);
            lines.extend(line);
        }
        lines
    }
    fn random_hash(input: Option<String>) -> Vec<u8> {
        match input {
            Some(input) => sha1::Sha1::digest(input.as_bytes()).to_vec(),
            None => sha1::Sha1::digest(b"random_data").to_vec(),
        }
    }
}
