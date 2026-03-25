use sha1::Digest;

pub struct ObjectHash {
    path : std::path::PathBuf,
    hash : sha1::Sha1,
    obj_type : String,
    content_length : u64
}

impl ObjectHash {
    pub fn build(path : std::path::PathBuf) -> anyhow::Result<ObjectHash> {
 
        
        Ok(ObjectHash {
           path : std::path::PathBuf::new(),
           hash : sha1::Sha1::new(),
           obj_type : String::new(),
           content_length : 0
        })
    }    
}