use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::EducationLevelSettings;

#[gamepacket(id = 137)]
#[derive(ProtoCodec)]
pub struct EducationSettingsPacket {
    pub education_level_settings: EducationLevelSettings,
}