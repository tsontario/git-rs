use crate::objects::object::{Object, ObjectType};
use crate::objects::object_hash::ObjectHash;
use crate::objects::tree;
use flate2::read::ZlibDecoder;
use std::io::{Read, Write};
use std::{fs, path};

/// Encapsulates access to the git object store on the filesystem.
///
/// Owns all object I/O. Other structs should use Store to access
/// objects rather than interacting with the filesystem directly.
pub struct Store {
    repo_dir: path::PathBuf,
}

impl Store {
    const DEFAULT_OBJ_PATH: &str = ".git/objects";
    const BUF_SIZE: usize = 8192;

    /// Creates a new Store rooted at the directory containing the `.git` directory
    pub fn new(repo_dir: path::PathBuf) -> anyhow::Result<Self> {
        let git_dir = repo_dir.join(".git");
        if !git_dir.exists() {
            return Err(anyhow::anyhow!(
                "Repository directory does not contain a .git directory"
            ));
        }
        Ok(Self { repo_dir })
    }

    /// Returns the path within the object store for the given object hash
    pub fn path_for_object(&self, obj_hash: &str) -> path::PathBuf {
        let (prefix, suffix) = obj_hash.split_at(2);
        self.objects_dir().join(prefix).join(suffix)
    }

    /// Loads an object from the object store by its hash.
    pub fn load_object(&self, obj_hash: &str) -> anyhow::Result<Object> {
        let decoded_object = self.decode_object(obj_hash)?;
        Object::build(decoded_object)
    }

    /// Absolute path to the root directory of the repository
    pub fn repo_dir(&self) -> &path::Path {
        &self.repo_dir
    }

    /// Writes a new object of type obj_type to the object store. Returns the hash of the object.
    /// The object is first written to a temporary file in the objects dir before being moved to its
    /// final location.
    ///
    /// # Arguments
    /// * `obj_type` - the type of the object to write
    /// * `reader` - the contents to be written to the store
    /// * `size` - the size of the object being read in bytes
    pub fn write_object(
        &self,
        obj_type: ObjectType,
        reader: &mut impl Read,
        size: usize,
    ) -> anyhow::Result<ObjectHash> {
        let mut tempfile = tempfile::Builder::new()
            .prefix("tmp_obj_")
            .tempfile_in(self.objects_dir())?;

        let obj_hash = ObjectHash::build(reader, &mut tempfile, obj_type, size)?;
        let obj_file_path = self.create_path_for_object(&obj_hash.hash)?;

        tempfile.persist(obj_file_path)?;
        Ok(obj_hash)
    }

    /// Reads the object data from the file pointed to by obj_hash and returns the decoded result
    pub fn decode_object(&self, obj_hash: &str) -> anyhow::Result<Vec<u8>> {
        let source_file = fs::File::open(self.path_for_object(obj_hash))?;
        let mut buffer = vec![0; Self::BUF_SIZE];
        let mut decoded_object = Vec::new();
        let mut zlib_decoder = ZlibDecoder::new(source_file);
        loop {
            let bytes_read = zlib_decoder.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            decoded_object.write_all(&buffer[0..bytes_read])?;
        }
        Ok(decoded_object)
    }

    /// Loads a tree, recusively expanding subtrees.
    pub fn load_tree_recursive(
        &self,
        obj_hash: &str,
        path_prefix: path::PathBuf,
    ) -> anyhow::Result<tree::Tree> {
        let Object::Tree(tree) = self.load_object(obj_hash)? else {
            return Err(anyhow::anyhow!("object is not a tree"));
        };
        let mut result: Vec<tree::TreeEntry> = vec![];
        for entry in tree.entries.iter() {
            if entry.object_type() == ObjectType::Tree {
                let subtree =
                    self.load_tree_recursive(&entry.hash, path_prefix.join(&entry.filename))?;
                result.extend(subtree.entries)
            } else {
                result.push(entry.clone());
            }
        }

        Ok(tree::Tree { entries: result })
    }

    fn objects_dir(&self) -> path::PathBuf {
        self.repo_dir.join(Store::DEFAULT_OBJ_PATH)
    }

    /// Creates the directory path for the given object hash and returns the path to the object
    fn create_path_for_object(&self, obj_hash: &str) -> anyhow::Result<path::PathBuf> {
        let path = self.path_for_object(obj_hash);
        let parent = path.parent().expect("object path always has a parent");
        fs::create_dir_all(parent)?;
        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::object::Object::Tree;
    use crate::objects::object::ObjectType;
    use tempfile::TempDir;

    #[test]
    fn test_write_object() {
        let (store, _tempdir) = build_store();
        let mut reader = b"hello world".as_slice();
        let reader_len = reader.len();
        let result = store.write_object(ObjectType::Blob, &mut reader, reader_len);
        result.unwrap();
    }

    #[test]
    fn load_tree_recursively_simple() {
        let (store, _tempdir) = build_store();
        let mut reader = b"hello world".as_slice();
        let reader_len = reader.len();
        let blob_hash = store
            .write_object(ObjectType::Blob, &mut reader, reader_len)
            .unwrap();
        let entry = tree::TreeEntry {
            mode: 100644,
            hash: blob_hash.hash,
            filename: "test.txt".to_string(),
            size: 11,
        };
        let tree = tree::Tree {
            entries: vec![entry],
        };

        let tree_hash = store
            .write_object(
                ObjectType::Tree,
                &mut tree.to_bytes().as_slice(),
                tree.to_bytes().len(),
            )
            .unwrap();
        let result = store.load_tree_recursive(&tree_hash.hash, path::PathBuf::new());
        assert_eq!(result.unwrap().entries[0].filename, "test.txt");
    }

    fn build_store() -> (Store, TempDir) {
        let tempdir = tempfile::tempdir().unwrap();
        fs::create_dir_all(tempdir.path().join(".git").join("objects")).unwrap();
        (Store::new(tempdir.path().to_path_buf()).unwrap(), tempdir)
    }
}
