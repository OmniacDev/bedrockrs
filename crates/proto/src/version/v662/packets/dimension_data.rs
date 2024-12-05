use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::DimensionDefinitionGroup;

#[gamepacket(id = 180)]
#[derive(ProtoCodec)]
pub struct DimensionDataPacket {
    pub dimension_definition_group: DimensionDefinitionGroup,
}