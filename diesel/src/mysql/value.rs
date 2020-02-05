use super::{Mysql, MysqlTypeMetadata};
use crate::backend::BinaryRawValue;

/// Raw mysql value as received from the database
#[derive(Copy, Clone, Debug)]
pub struct MysqlValue<'a> {
    raw: &'a [u8],
    tpe: MysqlTypeMetadata,
}

impl<'a> MysqlValue<'a> {
    pub(crate) fn new(raw: &'a [u8], tpe: MysqlTypeMetadata) -> Self {
        Self {
            raw,
            tpe: tpe.into(),
        }
    }

    /// Get the underlying raw byte representation
    pub fn as_bytes(&self) -> &[u8] {
        self.raw
    }

    /// Get the mysql type of the current value
    pub fn value_type(&self) -> MysqlTypeMetadata {
        self.tpe
    }
}

impl<'a> BinaryRawValue<'a> for Mysql {
    fn as_bytes(value: Self::RawValue) -> &'a [u8] {
        value.raw
    }
}
