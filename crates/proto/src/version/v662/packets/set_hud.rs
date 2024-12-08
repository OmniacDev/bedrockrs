use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::{HudElement, HudVisibility};

#[gamepacket(id = 308)]
#[derive(ProtoCodec)]
pub struct SetHudPacket {
    #[vec_repr(u32)]
    #[vec_endianness(var)]
    pub hud_elements_list: Vec<HudElement>,
    pub hud_visibility: HudVisibility,
}