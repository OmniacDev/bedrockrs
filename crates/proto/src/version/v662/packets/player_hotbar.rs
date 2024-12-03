use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::ContainerID;

#[gamepacket(id = 48)]
#[derive(ProtoCodec)]
pub struct PlayerHotbarPacket {
    #[endianness(var)]
    pub selected_slot: u32,
    pub container_id: ContainerID,
    pub should_select_slot: bool,
}