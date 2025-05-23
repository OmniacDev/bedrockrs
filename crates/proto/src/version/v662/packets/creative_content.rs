use super::super::types::NetworkItemInstanceDescriptor;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 145)]
#[derive(ProtoCodec, Clone, Debug)]
pub struct CreativeContentPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub write_entries: Vec<WriteEntry>,
}

#[derive(ProtoCodec, Clone, Debug)]
pub struct WriteEntry {
    #[endianness(var)]
    pub creative_net_id: u32,
    pub item_instance: NetworkItemInstanceDescriptor,
}
