use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
#[enum_repr(i64)]
#[enum_endianness(var)]
#[repr(i64)]
pub enum ActorBlockSyncMessageID {
    NONE = 0,
    CREATE = 1,
    DESTROY = 2,
}
