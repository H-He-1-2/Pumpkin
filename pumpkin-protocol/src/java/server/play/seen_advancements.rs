use pumpkin_data::packet::serverbound::PLAY_SEEN_ADVANCEMENTS;
use pumpkin_macros::java_packet;
use serde::Serialize;

#[derive(serde::Deserialize, Serialize)]
#[java_packet(PLAY_SEEN_ADVANCEMENTS)]
pub struct SSeenAdvancements {
    pub action_id: i32,
}
