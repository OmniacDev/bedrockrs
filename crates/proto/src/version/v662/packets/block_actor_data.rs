use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::NetworkBlockPosition;

#[gamepacket(id = 56)]
#[derive(ProtoCodec)]
pub struct BlockActorDataPacket {
    pub block_position: NetworkBlockPosition,
    #[nbt]
    pub actor_data_tags: nbtx::Value, // TODO: NBT Structure
}