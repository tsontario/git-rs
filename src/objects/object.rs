use std::fmt;
use std::fmt::Display;

pub struct Object {
    pub obj_type : ObjectType,
    pub content : Vec<u8>,
    pub size : usize,
}

impl Object {
    pub fn build(buf : Vec<u8>) -> Result<Object, anyhow::Error> {
        let null_pos = buf.iter().position(|&x| x == 0).ok_or_else(|| anyhow::anyhow!("malformed object header: no null byte found"))?;
        let header = String::from_utf8(buf[0..null_pos].to_vec())?;
        let (obj_type, size) = header.split_once(' ').ok_or_else(|| anyhow::anyhow!("malformed object header"))?;
        Ok(Object {
            obj_type : obj_type.parse::<ObjectType>()?,
            content : buf[null_pos+1..].to_vec(),
            size : size.parse::<usize>()?,
        })
    }
}

/// The type of Git object.
#[derive(Copy, Clone)]
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