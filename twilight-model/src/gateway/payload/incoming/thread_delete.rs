use crate::{
    channel::ChannelType,
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadDelete {
    pub guild_id: Id<GuildMarker>,
    pub id: Id<ChannelMarker>,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub parent_id: Id<ChannelMarker>,
}
