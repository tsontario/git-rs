use std::io::{Read, Write};
use flate2::read::ZlibDecoder;
use crate::objects::object_hash::{ObjectHash, ObjectType};

pub(crate) const DEFAULT_OBJ_PATH : &str = ".git/objects";

pub fn write_object(obj_type : ObjectType, reader : &mut impl Read, out_path : &std::path::Path, size : usize) -> anyhow::Result<ObjectHash> {
    std::fs::create_dir_all(out_path)?;
    let mut tempfile = tempfile::Builder::new()
        .prefix("tmp_obj_")
        .tempfile_in(out_path)?;

    let obj_hash = ObjectHash::build(reader, &mut tempfile, obj_type, size)?;

    let obj_file_path = create_path_for_object(&obj_hash, out_path)?;

    tempfile.persist(obj_file_path)?;
    Ok(obj_hash)
}

pub fn create_path_for_object(obj_hash : &ObjectHash, base_path : &std::path::Path) -> anyhow::Result<std::path::PathBuf> {
    let path = base_path.join(path_for_object(obj_hash));
    std::fs::create_dir_all(path.parent().unwrap())?;
    Ok(path)
}


pub fn path_for_object(obj_hash : &ObjectHash) -> std::path::PathBuf {
    let (prefix, suffix) = obj_hash.hash.split_at(2);
    std::path::Path::new(prefix).join(suffix)
}

pub fn load(reader : &mut impl Read, writer : &mut impl Write, buf_size : usize) -> anyhow::Result<()> {
    let mut buffer = vec![0; buf_size];
    let mut zlib_decoder = ZlibDecoder::new(reader);
    loop {
        let bytes_read = zlib_decoder.read(&mut buffer)?;
        if bytes_read == 0 { break }
        writer.write_all(&buffer[0..bytes_read])?;
    }
    Ok(())
}

#[test]
fn test_write_object() {
    let tempdir = tempfile::tempdir().unwrap();
    let mut reader = b"hello world".as_slice();
    let reader_len = reader.len();
    let result = write_object(ObjectType::Blob, &mut reader, tempdir.path(), reader_len);
    result.unwrap();
}