use std::io::{Read, Write};
use sha1::Digest;
use flate2::write::ZlibEncoder;
use std::fmt;
use std::fmt::Display;

/// The type of Git object.
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
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

/// A computed Git object hash with its SHA-1 hex digest.
pub struct ObjectHash {
    pub hash : String
}

impl ObjectHash {
    const DEFAULT_BUF_SIZE: usize = 8192;

    /// Hashes and zlib-compresses a Git object, writing the result to `writer`.
    ///
    /// Reads raw content from `reader`, prepends the standard Git object header
    /// (`"{type} {size}\0"`), and returns the SHA-1 hash of the full object.
    pub fn build(reader : &mut impl Read, writer : &mut impl Write, obj_type : ObjectType, size : u64) -> anyhow::Result<ObjectHash> {
        Self::build_with_buf_size(reader, writer, obj_type, size, Self::DEFAULT_BUF_SIZE)
    }

    /// Like [`build`](Self::build), but with a configurable read buffer size.
    pub fn build_with_buf_size(reader : &mut impl Read, writer : &mut impl Write, obj_type : ObjectType, size : u64, buf_size : usize) -> anyhow::Result<ObjectHash> {
        let mut buffer = vec![0; buf_size];
        let mut hasher = sha1::Sha1::new();
        let mut zlib_encoder = ZlibEncoder::new(writer, flate2::Compression::default());

        let header = format!("{} {}\0", obj_type, size).into_bytes();
        hasher.update(&header);
        zlib_encoder.write_all(&header)?;

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 { break }

            hasher.update(&buffer[0..bytes_read]);
            zlib_encoder.write_all(&buffer[0..bytes_read])?;
        }

        let hash = format!("{:x}", hasher.finalize());
        Ok(ObjectHash {
            hash
        })
    }
}