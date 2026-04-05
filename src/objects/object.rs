use crate::objects::{blob, tree};
use std::fmt;
use std::fmt::Display;
use std::path::PathBuf;

pub(crate) trait ObjectMeta {
    fn size(&self) -> usize;
    fn obj_type(&self) -> ObjectType;
    fn content(&self) -> Vec<u8>;
}

pub enum Object {
    Blob(blob::Blob),
    Tree(tree::Tree),
}

impl ObjectMeta for Object {
    fn size(&self) -> usize {
        match self {
            Object::Blob(blob) => blob.size,
            Object::Tree(tree) => tree.entries.iter().map(|e| e.size).sum(),
        }
    }

    fn obj_type(&self) -> ObjectType {
        match self {
            Object::Blob(_) => ObjectType::Blob,
            Object::Tree(_) => ObjectType::Tree,
        }
    }

    fn content(&self) -> Vec<u8> {
        match self {
            Object::Blob(blob) => blob.content.to_vec(),
            Object::Tree(tree) => tree
                .entries
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<String>>()
                .join("\n")
                .as_bytes()
                .to_vec(),
        }
    }
}

impl Object {
    pub fn build(buf: Vec<u8>) -> Result<Object, anyhow::Error> {
        let null_pos = buf
            .iter()
            .position(|&x| x == 0)
            .ok_or_else(|| anyhow::anyhow!("malformed object header: no null byte found"))?;
        let header = String::from_utf8(buf[0..null_pos].to_vec())?;
        let (raw_obj_type, size) = header
            .split_once(' ')
            .ok_or_else(|| anyhow::anyhow!("malformed object header"))?;

        let obj_type = raw_obj_type.trim().parse::<ObjectType>()?;
        match obj_type {
            ObjectType::Blob => {
                let blob = blob::Blob {
                    obj_type: obj_type,
                    content: buf[null_pos + 1..].to_vec(),
                    size: size.parse::<usize>()?,
                };
                Ok(Object::Blob(blob))
            }
            ObjectType::Tree => {
                let entries = tree::TreeEntry::parse(&buf[null_pos + 1..])?;
                let tree = tree::Tree { entries };
                Ok(Object::Tree(tree))
            }
            ObjectType::Commit => Err(anyhow::anyhow!("unknown object type: commit")),
        }
    }
}

/// The type of Git object.
#[derive(Copy, Clone, PartialEq)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

impl std::str::FromStr for ObjectType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blob" => Ok(ObjectType::Blob),
            "tree" => Ok(ObjectType::Tree),
            "commit" => Ok(ObjectType::Commit),
            _ => Err(anyhow::anyhow!("unknown object type: {}", s)),
        }
    }
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectType::Blob => write!(f, "blob"),
            ObjectType::Tree => write!(f, "tree"),
            ObjectType::Commit => write!(f, "commit"),
        }
    }
}
