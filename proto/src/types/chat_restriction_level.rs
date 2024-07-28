use std::io::Cursor;
use std::sync::Arc;

use bedrockrs_core::LE;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;

#[derive(Debug, Clone)]
pub enum ChatRestrictionLevel {
    None,
    Dropped,
    Disabled,
}

impl ProtoCodec for ChatRestrictionLevel {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        let int = match self {
            ChatRestrictionLevel::None => 0x00,
            ChatRestrictionLevel::Dropped => 0x01,
            ChatRestrictionLevel::Disabled => 0x02,
        };

        match LE::<u8>::new(int).write(stream) {
            Ok(_) => Ok(()),
            Err(e) => Err(ProtoCodecError::IOError(Arc::new(e))),
        }
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        todo!()
    }
}