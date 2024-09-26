use crate::byteorder::{ProtoCodecBE, ProtoCodecLE, ProtoCodecVAR};
use crate::error::ProtoCodecError;
use crate::ProtoCodec;
use std::io::Cursor;

macro_rules! impl_proto_option {
    ($name:ident) => {
        impl<T: $name> $name for Option<T> {
            fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError>
            where
                Self: Sized,
            {
                match self {
                    Some(v) => {
                        bool::proto_serialize(*true, buf)?;
                        v.proto_serialize(buf)?;
                    }
                    None => bool::proto_serialize(*false, buf)?,
                }

                Ok(())
            }

            fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>
            where
                Self: Sized,
            {
                Ok(match bool::proto_deserialize(stream)? {
                    false => None,
                    true => Some(T::proto_deserialize(stream)?),
                })
            }
        }
    };
}

impl_proto_option!(ProtoCodec);
impl_proto_option!(ProtoCodecLE);
impl_proto_option!(ProtoCodecBE);
impl_proto_option!(ProtoCodecVAR);
