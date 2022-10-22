use crate::model::OpType;
use crate::model::OwnedEntry;

pub struct WriteBatch {
    rep: Vec<OwnedEntry>,
}

impl WriteBatch {
    pub fn new() -> WriteBatch {
        WriteBatch { rep: Vec::new() }
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) {
        self.rep.push(OwnedEntry {
            op_type: OpType::Put,
            key: key.to_vec(),
            value: Some(value.to_vec()),
            ts: Some(0),
        })
    }

    pub fn delete(&mut self, key: &[u8]) {
        self.rep.push(OwnedEntry {
            op_type: OpType::Del,
            key: key.to_vec(),
            value: None,
            ts: Some(0),
        })
    }
}
