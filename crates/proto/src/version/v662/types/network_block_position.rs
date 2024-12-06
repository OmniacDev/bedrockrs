use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct NetworkBlockPosition {
    #[endianness(var)]
    pub x: i32,
    #[endianness(var)]
    pub y: u32,
    #[endianness(var)]
    pub z: i32,
}