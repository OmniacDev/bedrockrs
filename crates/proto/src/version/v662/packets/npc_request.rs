use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::ActorRuntimeID;

#[derive(ProtoCodec)]
#[enum_repr(i8)]
#[repr(i8)]
enum RequestType {
    SetActions = 0,
    ExecuteAction = 1,
    ExecuteClosingCommands = 2,
    SetName = 3,
    SetSkin = 4,
    SetInteractText = 5,
    ExecuteOpeningCommands = 6,
}

#[gamepacket(id = 98)]
#[derive(ProtoCodec)]
pub struct NpcRequestPacket {
    pub npc_runtime_id: ActorRuntimeID,
    pub request_type: RequestType,
    pub actions: String,
    pub action_index: i8,
    pub scene_name: String,
}