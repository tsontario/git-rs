use std::cmp::PartialEq;
use crate::objects::object_hash::ObjectHash;
use crate::objects::tree::{Tree, TreeEntry};
use crate::objects::store;
use crate::objects::object;

pub fn traverse(hash: String, path_prefix: std::path::PathBuf) -> anyhow::Result<Vec<TreeEntry>> {
        let obj_hash = ObjectHash{hash};
        let path = store::path_for_object(&obj_hash);
        let mut reader = std::fs::File::open(path)?;
        let mut buffer : Vec<u8> = Vec::with_capacity(8096);
        store::load(&mut reader, &mut buffer, 8096)?;
        let mut entries = TreeEntry::parse(&buffer)?;

        let mut expansions : Vec<usize> = Vec::new();
        for i in 0..entries.len() {
            if entries[i].object_type() == object::ObjectType::Tree {
                expansions.push(i);
            }
        }

        for expansion in expansions {
            let path_prefix = std::path::Path::new(&path_prefix).join(&entries[expansion].filename);
            let expanded_entries = traverse(entries[expansion].hash.clone(), path_prefix);
            entries.splice(expansion ..expansion + 1, expanded_entries.unwrap().into_iter());
        }

        Ok(entries)
}
