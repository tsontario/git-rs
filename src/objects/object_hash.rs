use std::io::{Read, Write};
use sha1::Digest;
use flate2::write::ZlibEncoder;
use crate::objects::object::ObjectType;



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
    pub fn build(reader : &mut impl Read, writer : &mut impl Write, obj_type : ObjectType, size : usize) -> anyhow::Result<ObjectHash> {
        Self::build_with_buf_size(reader, writer, obj_type, size, Self::DEFAULT_BUF_SIZE)
    }

    /// Like [`build`](Self::build), but with a configurable read buffer size.
    pub fn build_with_buf_size(reader : &mut impl Read, writer : &mut impl Write, obj_type : ObjectType, size : usize, buf_size : usize) -> anyhow::Result<ObjectHash> {
        let mut buffer = vec![0; buf_size];
        let mut hasher = sha1::Sha1::new();
        let mut zlib_encoder = ZlibEncoder::new(writer, flate2::Compression::default());

        let header = format!("{} {}\0", obj_type, size).into_bytes();
        hasher.update(&header);
        zlib_encoder.write_all(&header)?;

        let mut total_bytes_read = 0;
        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 { break }
            total_bytes_read += bytes_read;

            hasher.update(&buffer[0..bytes_read]);
            zlib_encoder.write_all(&buffer[0..bytes_read])?;
        }

        if total_bytes_read != size as usize {
            return Err(anyhow::anyhow!("Read {} bytes, expected {}", total_bytes_read, size));
        }

        zlib_encoder.finish()?;
        let hash = format!("{:x}", hasher.finalize());
        Ok(ObjectHash {
            hash
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_object_hash() {
        let obj_type = ObjectType::Blob;
        let size = 11;
        let mut reader = b"hello world".as_slice();
        let mut writer = Vec::new();

        let object_hash = ObjectHash::build(&mut reader, &mut writer, obj_type, size).unwrap();

        let expected_hash_input = "blob 11\0hello world";
        let expected_hash = format!("{:x}", sha1::Sha1::digest(expected_hash_input.as_bytes())).to_string();
        assert_eq!(object_hash.hash, expected_hash);

        let expected_contents = Vec::new();
        let mut zlib_encoder = ZlibEncoder::new(expected_contents, flate2::Compression::default());
        zlib_encoder.write_all(expected_hash_input.as_bytes()).unwrap();
        let expected_contents = zlib_encoder.finish().unwrap();
        assert_eq!(writer, expected_contents);
    }

    #[test]
    fn test_build_object_bigger_than_buf_size() {
        let buf_size = 1;
        let obj_type = ObjectType::Blob;
        let size = 11;
        let mut reader = b"hello world".as_slice();
        let mut writer = Vec::new();

        let object_hash = ObjectHash::build_with_buf_size(&mut reader, &mut writer, obj_type, size, buf_size).unwrap();

        let expected_hash_input = "blob 11\0hello world";
        let expected_hash = format!("{:x}", sha1::Sha1::digest(expected_hash_input.as_bytes())).to_string();
        assert_eq!(object_hash.hash, expected_hash);

        let expected_contents = Vec::new();
        let mut zlib_encoder = ZlibEncoder::new(expected_contents, flate2::Compression::default());
        zlib_encoder.write_all(expected_hash_input.as_bytes()).unwrap();
        let expected_contents = zlib_encoder.finish().unwrap();
        assert_eq!(writer, expected_contents);
    }

    #[test]
    fn test_build_size_mismatch_returns_error() {
        let obj_type = ObjectType::Blob;
        let size = 5; // declared size != actual content length (11)
        let mut reader = b"hello world".as_slice();
        let mut writer = Vec::new();

        let result = ObjectHash::build(&mut reader, &mut writer, obj_type, size);
        assert!(result.is_err());
    }

    #[test]
    fn test_build_empty_object() {
        let obj_type = ObjectType::Blob;
        let size = 0;
        let mut reader = b"".as_slice();
        let mut writer = Vec::new();

        let object_hash = ObjectHash::build(&mut reader, &mut writer, obj_type, size).unwrap();

        let expected_hash_input = "blob 0\0";
        let expected_hash = format!("{:x}", sha1::Sha1::digest(expected_hash_input.as_bytes())).to_string();
        assert_eq!(object_hash.hash, expected_hash);

        let expected_contents = Vec::new();
        let mut zlib_encoder = ZlibEncoder::new(expected_contents, flate2::Compression::default());
        zlib_encoder.write_all(expected_hash_input.as_bytes()).unwrap();
        let expected_contents = zlib_encoder.finish().unwrap();
        assert_eq!(writer, expected_contents);
    }
}