use super::update_presence::UpdatePresencePayload;
use crate::gateway::{intents::Intents, opcode::OpCode, ShardId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Identify {
    pub d: IdentifyInfo,
    pub op: OpCode,
}

impl Identify {
    pub const fn new(info: IdentifyInfo) -> Self {
        Self {
            d: info,
            op: OpCode::Identify,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IdentifyInfo {
    pub compress: bool,
    pub presence: Option<UpdatePresencePayload>,
    pub properties: IdentifyProperties,
    pub token: String,
    #[serde(flatten)]
    pub extra_info: IdentifyExtraInfo,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IdentifyExtraInfo {
    Bot {
        intents: Intents,
        large_threshold: u64,
        shard: Option<ShardId>,
    },
    User {
        capabilities: u16,
        client_state: IdentifyClientState,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IdentifyClientState {
    pub api_code_version: i8,
    pub guild_versions: IdentifyGuildVersions,
    pub highest_last_message_id: String,
    pub private_channels_version: String,
    pub read_state_version: u8,
    pub user_guild_settings_version: i8,
    pub user_settings_version: i8,
}

impl Default for IdentifyClientState {
    fn default() -> Self {
        Self {
            api_code_version: 0,
            guild_versions: IdentifyGuildVersions {},
            highest_last_message_id: String::from("0"),
            private_channels_version: String::from("0"),
            read_state_version: 0,
            user_guild_settings_version: -1,
            user_settings_version: -1,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IdentifyGuildVersions {}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IdentifyProperties {
    pub browser: String,
    pub browser_user_agent: String,
    pub browser_version: String,
    pub client_build_number: u32,
    pub client_event_source: Option<u8>,
    pub device: String,
    pub os: String,
    pub os_version: String,
    pub referrer: String,
    pub referrer_current: String,
    pub referrering_domain: String,
    pub referrering_domain_current: String,
    pub release_channel: String,
    pub system_locale: String,
}

impl IdentifyProperties {
    pub fn new(
        _browser: impl Into<String>,
        _device: impl Into<String>,
        _os: impl Into<String>,
    ) -> Self {
        Self::default()
    }
}

impl Default for IdentifyProperties {
    fn default() -> Self {
        Self {
            browser: String::from("Chrome"),
            browser_user_agent: String::from("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36"),
            browser_version: String::from("109.0.0.0"),
            client_build_number: 175_627,
            client_event_source: None,
            device: String::from(""),
            os: String::from("Linux"),
            os_version: String::new(),
            referrer: String::new(),
            referrer_current: String::new(),
            referrering_domain: String::new(),
            referrering_domain_current: String::new(),
            release_channel: String::from("stable"),
            system_locale: String::from("en-US"),
        }
    }
}
