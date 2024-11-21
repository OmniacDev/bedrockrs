use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
    Count = 4,
    Unknown = 5,
}