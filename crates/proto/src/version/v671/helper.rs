use crate::helper::ProtoHelper;
use crate::version::v729::gamepackets::GamePackets;

pub struct ProtoHelperV671;

impl ProtoHelper for ProtoHelperV671 {
    type GamePacketType = GamePackets;
}