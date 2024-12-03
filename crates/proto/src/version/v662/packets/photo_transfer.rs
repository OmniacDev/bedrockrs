use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::PhotoType;

#[gamepacket(id = 99)]
#[derive(ProtoCodec)]
pub struct PhotoTransferPacket {
    pub photo_name: String,
    pub photo_data: String,
    pub book_id: String,
    pub photo_type: PhotoType,
    pub source_type: PhotoType,
    #[endianness(le)]
    pub owner_id: i64,
    pub new_photo_name: String,
}