use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub struct WebSocketPacketData {
    pub web_socket_server_uri: String,
}