use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(i16)]
#[enum_endianness(le)]
#[repr(i16)]
pub enum SpawnBiomeType {
    Default = 0,
    UserDefined = 1,
}
