use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct ChunkPos {
    #[endianness(var)]
    pub x: i32,
    #[endianness(var)]
    pub z: i32,
}