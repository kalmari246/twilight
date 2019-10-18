//! Models built for utilizing efficient caching.

mod emoji;
mod guild;
mod member;
mod message;
mod presence;
mod voice_state;

pub use self::{
    emoji::CachedEmoji,
    guild::CachedGuild,
    member::CachedMember,
    message::CachedMessage,
    presence::CachedPresence,
    voice_state::CachedVoiceState,
};

#[cfg(tests)]
mod tests {
    #[test]
    fn test_reexports() {
        use super::{CachedEmoji, CachedGuild, CachedMember, CachedPresence, CachedVoiceState};
    }
}
