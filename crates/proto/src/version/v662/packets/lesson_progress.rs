use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::LessonAction;

#[gamepacket(id = 183)]
#[derive(ProtoCodec)]
pub struct LessonProgressPacket {
    pub lesson_action: LessonAction,
    #[endianness(var)]
    pub score: i32,
    pub activity_id: String,
}