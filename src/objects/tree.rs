use std::fmt::{Display, Formatter};
use crate::objects::object::ObjectType;
use crate::objects::utils;

pub struct Tree {
    pub entries: Vec<TreeEntry>,
}

pub struct TreeEntry {
    pub mode: u32,
    pub filename: String,
    pub hash: String,
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
            040000 => Ok(Mode::Directory),
            120000 => Ok(Mode::SymLink),
            120001 => Ok(Mode::GitLink),
            _ => Err(anyhow::anyhow!("unknown mode: {}", mode)),
        }
    }

    pub fn object_type(&self) -> ObjectType {
        match self {
            Mode::RegularFile  | Mode::ExecutableFile | Mode::SymLink => ObjectType::Blob,
            Mode::Directory => ObjectType::Tree,
            Mode::GitLink => ObjectType::Commit,
        }
    }
}

impl TreeEntry {
    // Parses a slice of bytes into a vector of tree entries.
    pub fn parse(bytes: &[u8]) -> anyhow::Result<Vec<Self>> {
        let mut offset : usize = 0;
        let mut entries : Vec<Self> = Vec::new();
        while offset < bytes.len() {
            let (entry, new_offset) = Self::parse_one(&bytes[offset..])?;
            entries.push(entry);
            offset += new_offset;
        }
        Ok(entries)
    }

    // Parses a single tree entry from a slice of bytes.
    // Returns the parsed entry and the offset of the (the possible) next entry.
    // It is the caller's responsibility to check if the offset is valid.
    fn parse_one(bytes: &[u8]) -> anyhow::Result<(Self, usize)> {
        let null_pos = bytes.iter().position(|&b| { b == 0 }).unwrap();
        let space_pos = bytes.iter().position(|&b| { b == b' ' }).unwrap();

        let mode = std::str::from_utf8(&bytes[..space_pos]).unwrap().parse::<u32>()?;
        let filename = String::from_utf8(bytes[space_pos+1..null_pos].to_vec())?;
        let hash = utils::bytes_to_string(&bytes[null_pos+1..null_pos+21]);

        // null_pos + 22 is the offset of the next entry
        Ok((TreeEntry{mode, filename, hash}, null_pos + 21))
    }

    fn object_type(&self) -> ObjectType {
        Mode::from_u32(self.mode).unwrap().object_type()
    }
}

impl Display for TreeEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:06} {} {}\t{}", self.mode, self.object_type(), self.hash, self.filename)
    }
}

#[cfg(test)]
mod tests {
    use sha1::Digest;
    use crate::objects::utils::bytes_to_string;
    use super::*;

    #[test]
    fn parse_single_tree_entry() {
        let hash = random_hash(None);
        let mut bytes = b"100644 file1.txt\0".to_vec();
        bytes.extend_from_slice(&hash);
        let entries = TreeEntry::parse(&bytes).unwrap();
        assert_eq!(entries.len(), 1);
        let entry = entries.get(0).unwrap();
        assert_eq!(entry.mode, 100644);
        assert_eq!(entry.filename, "file1.txt");
        assert_eq!(entry.hash, bytes_to_string(&hash));
    }

    #[test]
    fn parse_multiple_tree_entry() {
        let entries = build_tree_lines(3);
        let parsed_entries = TreeEntry::parse(&entries).unwrap();
        assert_eq!(parsed_entries.len(), 3);
        for i in 0..3 {
            let entry = parsed_entries.get(i).unwrap();
            let expected_hash = random_hash(Some(format!("{}", i)));
            assert_eq!(entry.mode, 100644);
            assert_eq!(entry.filename, format!("file{}.txt", i));
            assert_eq!(entry.hash, bytes_to_string(&expected_hash));
        }
    }

    fn build_tree_lines(len : usize) -> Vec<u8> {
        let mut lines = Vec::new();
        for i in 0..len {
            let hash = random_hash(Some(format!("{}", i)));
            let mut line = format!("100644 file{}.txt\0", i).as_bytes().to_vec();
            line.extend_from_slice(&hash);
            lines.extend(line);
        }
        lines
    }
    fn random_hash(input : Option<String>) -> Vec<u8> {
        match input {
            Some(input) => sha1::Sha1::digest(input.as_bytes()).to_vec(),
            None => sha1::Sha1::digest(b"random_data").to_vec()
        }
    }
}