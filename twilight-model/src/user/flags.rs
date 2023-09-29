use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    /// Undocumented flags from [https://github.com/lewisakura/discord-flags/blob/master/flags/user.json].
    pub struct UserFlags: u64 {
        /// Discord Employee.
        const STAFF = 1;
        /// Partnered server owner.
        const PARTNER = 1 << 1;
        /// HypeSquad events member.
        const HYPESQUAD = 1 << 2;
        /// Bug hunter level 1.
        const BUG_HUNTER_LEVEL_1 = 1 << 3;
        /// User has SMS 2FA enabled.
        const MFA_SMS = 1 << 4;
        /// Unknown. Presumably some sort of Discord Nitro promotion that the user dismissed.
        const PREMIUM_PROMO_DISMISSED = 1 << 5;
        /// House Bravery member.
        const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;
        /// House Brilliance member.
        const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;
        /// House Balance member.
        const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;
        /// Early Nitro supporter.
        const PREMIUM_EARLY_SUPPORTER = 1 << 9;
        /// User is in a team.
        const TEAM_PSEUDO_USER = 1 << 10;
        /// An internal flag accidentally leaked to the client's private flags. Relates to partner/verification applications but nothing else is known.
        const INTERNAL_APPLICATION = 1 << 11;
        /// Account is a Discord system account.
        const SYSTEM = 1 << 12;
        /// User has unread messages from Discord.
        const HAS_UNREAD_URGENT_MESSAGES = 1 << 13;
        /// Bug hunter level 2.
        const BUG_HUNTER_LEVEL_2 = 1 << 14;
        /// Unused. User was deleted for being underage.
        const UNDERAGE_DELETED = 1 << 15;
        /// Verified bot.
        const VERIFIED_BOT = 1 << 16;
        /// Early verified bot developer.
        const VERIFIED_DEVELOPER = 1 << 17;
        /// Moderator Programs Alumni
        #[deprecated(since = "0.14.0", note = "use `MODERATOR_PROGRAMS_ALUMNI`")]
        const CERTIFIED_MODERATOR = 1 << 18;
        /// Moderator Programs Alumni
        const MODERATOR_PROGRAMS_ALUMNI = 1 << 18;
        /// Bot uses only HTTP interactions and is shown in the online member
        /// list.
        const BOT_HTTP_INTERACTIONS = 1 << 19;
        /// User is marked as a spammer.
        const SPAMMER = 1 << 20;
        /// Forcefully disables Nitro features.
        const DISABLE_PREMIUM = 1 << 21;
        /// User is an [Active Developer].
        ///
        /// [Active Developer]: https://support-dev.discord.com/hc/articles/10113997751447
        const ACTIVE_DEVELOPER = 1 << 22;
        /// Account has a high global ratelimit.
        const HIGH_GLOBAL_RATE_LIMIT = 1 << 33;
        /// Account has been deleted.
        const DELETED = 1 << 34;
        /// Account has been disabled for suspicious activity.
        const DISABLED_SUSPICIOUS_ACTIVITY = 1 << 35;
        /// Account was deleted by the user.
        const SELF_DELETED = 1 << 36;
        /// User has a premium discriminator.
        const PREMIUM_DISCRIMINATOR = 1 << 37;
        /// User has used the desktop client.
        const USED_DESKTOP_CLIENT = 1 << 38;
        /// User has used the web client.
        const USED_WEB_CLIENT = 1 << 39;
        /// User has used the mobile client.
        const USED_MOBILE_CLIENT = 1 << 40;
        /// User is currently temporarily or permanently disabled.
        const DISABLED = 1 << 41;
        /// User has a verified email.
        const VERIFIED_EMAIL = 1 << 43;
        /// User account is quarantined.
        const QUARANTINED = 1 << 44;
        /// User is a collaborator and has staff permissions.
        const COLLABORATOR = 1 << 50;
        /// User is a restricted collaborator and has staff permissions.
        const RESTRICTED_COLLABORATOR = 1 << 51;
    }
}

impl<'de> Deserialize<'de> for UserFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for UserFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]

    use super::UserFlags;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_impl_all, const_assert_eq};
    use std::{
        fmt::{Binary, Debug, LowerHex, Octal, UpperHex},
        hash::Hash,
        ops::{
            BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub, SubAssign,
        },
    };

    assert_impl_all!(
        UserFlags: Binary,
        BitAnd,
        BitAndAssign,
        BitOr,
        BitOrAssign,
        BitXor,
        BitXorAssign,
        Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Extend<UserFlags>,
        FromIterator<UserFlags>,
        Hash,
        LowerHex,
        Not,
        Octal,
        Ord,
        PartialEq,
        PartialOrd,
        Send,
        Serialize,
        Sub,
        SubAssign,
        Sync,
        UpperHex
    );

    const_assert_eq!(UserFlags::STAFF.bits(), 1);
    const_assert_eq!(UserFlags::PARTNER.bits(), 1 << 1);
    const_assert_eq!(UserFlags::HYPESQUAD.bits(), 1 << 2);
    const_assert_eq!(UserFlags::BUG_HUNTER_LEVEL_1.bits(), 1 << 3);
    const_assert_eq!(UserFlags::HYPESQUAD_ONLINE_HOUSE_1.bits(), 1 << 6);
    const_assert_eq!(UserFlags::HYPESQUAD_ONLINE_HOUSE_2.bits(), 1 << 7);
    const_assert_eq!(UserFlags::HYPESQUAD_ONLINE_HOUSE_3.bits(), 1 << 8);
    const_assert_eq!(UserFlags::PREMIUM_EARLY_SUPPORTER.bits(), 1 << 9);
    const_assert_eq!(UserFlags::TEAM_PSEUDO_USER.bits(), 1 << 10);
    const_assert_eq!(UserFlags::BUG_HUNTER_LEVEL_2.bits(), 1 << 14);
    const_assert_eq!(UserFlags::VERIFIED_BOT.bits(), 1 << 16);
    const_assert_eq!(UserFlags::VERIFIED_DEVELOPER.bits(), 1 << 17);
    const_assert_eq!(UserFlags::CERTIFIED_MODERATOR.bits(), 1 << 18);
    const_assert_eq!(UserFlags::MODERATOR_PROGRAMS_ALUMNI.bits(), 1 << 18);
    const_assert_eq!(UserFlags::BOT_HTTP_INTERACTIONS.bits(), 1 << 19);
    const_assert_eq!(UserFlags::ACTIVE_DEVELOPER.bits(), 1 << 22);

    #[test]
    fn serde() {
        serde_test::assert_tokens(
            &UserFlags::PARTNER,
            &[Token::U64(UserFlags::PARTNER.bits())],
        );
        // Deserialization truncates unknown bits.
        serde_test::assert_de_tokens(&UserFlags::empty(), &[Token::U64(1 << 63)]);
    }
}
