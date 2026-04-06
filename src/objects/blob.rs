use crate::objects::object::ObjectType;

pub struct Blob {
    pub obj_type: ObjectType,
    pub content: Vec<u8>,
    pub size: usize,
}
