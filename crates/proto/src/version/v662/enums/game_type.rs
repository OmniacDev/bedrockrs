use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum GameType {
    Undefined = -1,
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Default = 5,
    Spectator = 6,
}

impl GameType {
    pub const WORLD_DEFAULT: GameType = GameType::Survival;
}