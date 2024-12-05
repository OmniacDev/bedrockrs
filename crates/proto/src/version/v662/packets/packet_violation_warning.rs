use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::enums::{MinecraftPacketIds, PacketViolationSeverity, PacketViolationType};

#[gamepacket(id = 156)]
#[derive(ProtoCodec)]
pub struct PacketViolationWarningPacket {
    pub violation_type: PacketViolationType,
    pub violation_severity: PacketViolationSeverity,
    pub violating_packet_id: MinecraftPacketIds,
    pub violation_context: String,
}