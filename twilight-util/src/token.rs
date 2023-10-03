//! Provides a method for parsing a token.

use base64::{
    alphabet,
    engine::general_purpose::{self, GeneralPurpose},
    Engine,
};
use std::{error, fmt, str};
use twilight_model::id::{marker::UserMarker, Id};

/// Not a token.
#[derive(Debug)]
pub struct NotAToken(());

impl fmt::Display for NotAToken {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("not a token")
    }
}

impl error::Error for NotAToken {}

/// Decode base64 to a byte array of max size `N`.
fn base64_decode<const N: usize>(base64: &str) -> Option<([u8; N], usize)> {
    const SLOPPY_URL_SAFE_NO_PAD: GeneralPurpose = GeneralPurpose::new(
        &alphabet::URL_SAFE,
        general_purpose::NO_PAD.with_decode_allow_trailing_bits(true),
    );

    let mut bytes = [0; N];
    let len = SLOPPY_URL_SAFE_NO_PAD
        .decode_slice_unchecked(base64, &mut bytes)
        .ok()?;

    Some((bytes, len))
}

/// Parse a token into each part.
fn parse_inner(token: &str) -> Option<(Id<UserMarker>, u64, &str)> {
    const TOKEN_EPOCH: u64 = 1_293_840_000;

    let mut iter = token.splitn(3, '.');
    let (id, timestamp, hmac) = (iter.next()?, iter.next()?, iter.next()?);

    let (bytes, len) = base64_decode::<19>(id)?;
    let id = str::from_utf8(&bytes[..len]).ok()?.parse().ok()?;

    let (bytes, _len) = base64_decode::<8>(timestamp)?;
    let timestamp = u64::from_ne_bytes(bytes).checked_add(TOKEN_EPOCH)?;

    Some((id, timestamp, hmac))
}

/// Parse a token into each part.
pub fn parse(token: &str) -> Result<(Id<UserMarker>, u64, &str), NotAToken> {
    parse_inner(token).ok_or(NotAToken(()))
}
