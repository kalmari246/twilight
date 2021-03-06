use crate::{
    id::{marker::GuildMarker, Id},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BanRemove {
    pub guild_id: Id<GuildMarker>,
    pub user: User,
}
