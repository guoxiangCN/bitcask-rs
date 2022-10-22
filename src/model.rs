use crate::errors::DBResult;
use std::io::Write;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum OpType {
    #[default]
    Put,
    Del,
}

impl From<u8> for OpType {
    fn from(value: u8) -> Self {
        match value {
            0 => OpType::Put,
            1 => OpType::Del,
            _ => panic!("invalid u8"),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct OwnedEntry {
    pub(crate) op_type: OpType,
    pub(crate) key: Vec<u8>,
    pub(crate) value: Option<Vec<u8>>,
    pub(crate) ts: Option<u64>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct RefEntry<'a, 'b> {
    pub(crate) op_type: OpType,
    pub(crate) key: &'a [u8],
    pub(crate) value: Option<&'b [u8]>,
    pub(crate) ts: Option<u64>,
}

impl OwnedEntry {
    pub(crate) fn as_ref_entry<'a, 'b, 'c>(&'c self) -> RefEntry<'a, 'b>
    where
        'c: 'a,
        'c: 'b,
    {
        RefEntry::<'a, 'b> {
            op_type: self.op_type.clone(),
            key: &self.key,
            value: self.value.as_ref().map(|x| x.as_ref()),
            ts: self.ts.clone(),
        }
    }

    pub(crate) fn decode_from_bytes(bytes: &[u8], _verify_crc: bool) -> DBResult<OwnedEntry> {
        // crc(4)+ts(8)+keysz(4)+valsz(4)+key+optype(1)+val
        assert!(bytes.len() > 20);
        let mut entry = OwnedEntry::default();
        // skip crc todo
        entry.ts = Some(u64::from_be_bytes((bytes[4..12]).try_into().unwrap()));
        // keysz
        let keysz = u32::from_be_bytes(bytes[12..16].try_into().unwrap());
        let valsz = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
        assert!(bytes.len() >= (20 + keysz + valsz + 1) as usize);
        // read key
        entry
            .key
            .extend_from_slice(&bytes[20..(20 + keysz as usize)]);
        entry.op_type = OpType::from(bytes[20 + keysz as usize]);
        if entry.op_type == OpType::Del {
            assert!(valsz == 0);
            entry.value = None;
        } else {
            let mut val = vec![];
            val.extend_from_slice(&bytes[(20 + keysz + 1) as usize..]);
            entry.value = Some(val);
        }
        Ok(entry)
    }
}

impl<'a, 'b> RefEntry<'a, 'b> {
    /// |crc|ts|ksz|vsz|key|op|value|
    pub(crate) fn encode_to_bytes(&self) -> Vec<u8> {
        assert!(self.key.len() < 2 ^ 32);
        assert!(self.value.as_ref().map_or(0, |x| x.len()) < 2 ^ 32);
        if self.op_type == OpType::Del {
            assert!(self.value.is_none());
        }

        let mut data = vec![];
        // CRC32
        data.write(&[0, 0, 0, 0]).unwrap();

        // ts
        let ts = self.ts.clone().unwrap_or(0);
        let ts_bytes = ts.to_be_bytes();
        assert!(ts_bytes.len() == 8);
        data.write(&ts_bytes).unwrap();

        // keysz
        let keysz = (self.key.len() as u32).to_be_bytes();
        assert!(keysz.len() == 4);
        data.write(&keysz).unwrap();

        // valsz
        let valsz = self.value.as_ref().map_or(0, |x| x.len());
        let valsz = (valsz as u32).to_be_bytes();
        assert!(valsz.len() == 4);
        data.write(&valsz).unwrap();

        // key
        data.write_all(self.key).unwrap();
        // op_type
        data.push(self.op_type as u8);
        // value
        if self.value.is_some() {
            // Clone a reference not all data.
            data.write_all(self.value.clone().unwrap()).unwrap();
        }
        data
    }
}

pub(crate) trait EntryConsumer {
    fn consume(&self, entry: RefEntry);
}
