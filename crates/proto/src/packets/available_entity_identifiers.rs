use crate::types::entity_info::EntityInfoList;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 119)]
#[derive(ProtoCodec, Debug, Clone)]
pub struct AvailableEntityIdentifiersPacket {
    #[nbt]
    entity_info_list: EntityInfoList,
}