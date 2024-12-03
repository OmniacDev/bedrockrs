use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::{BaseGameVersion, Experiments};

#[derive(ProtoCodec)]
struct PackEntry {
    pub id: String,
    pub version: String,
    pub sub_pack_name: String,
}

#[gamepacket(id = 7)]
#[derive(ProtoCodec)]
pub struct ResourcePackStackPacket {
    pub texture_pack_required: bool,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub addon_list: Vec<PackEntry>,
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub texture_pack_list: Vec<PackEntry>,
    pub base_game_version: BaseGameVersion,
    pub experiments: Experiments,
}