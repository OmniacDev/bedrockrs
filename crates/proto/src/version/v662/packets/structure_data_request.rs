use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::StructureTemplateRequestOperation;
use crate::version::v662::types::{NetworkBlockPosition, StructureSettings};

#[gamepacket(id = 132)]
#[derive(ProtoCodec)]
pub struct StructureDataRequestPacket {
    pub structure_name: String,
    pub structure_position: NetworkBlockPosition,
    pub structure_settings: StructureSettings,
    pub requested_operation: StructureTemplateRequestOperation,
}