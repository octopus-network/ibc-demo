pub mod channel;
pub mod client;
pub mod connection;
pub mod packet;
pub mod port;

use std::str::FromStr;

use crate::error::{ValidationError, ValidationKind};
use tendermint::account::Id as AccountId;

fn get_dummy_account_id_raw() -> String {
    "0CDA3F47EF3C4906693B170EF650EB968C5F4B2C".to_string()
}

pub fn get_dummy_account_id() -> AccountId {
    AccountId::from_str(&get_dummy_account_id_raw()).unwrap()
}

/// Bails from the current function with the given error kind.
macro_rules! bail {
    ($kind:expr) => {
        return Err($kind.into());
    };
}

/// Path separator (ie. forward slash '/')
const PATH_SEPARATOR: char = '/';
const VALID_SPECIAL_CHARS: &str = "._+-#[]<>";

pub fn validate_channel_identifier(id: &str) -> &str {
    let _re = validate_identifier(id, 10, 64);
    if _re.is_err() {
        panic!(format!("Invalide channel identifier: {}", id));
    }

    return id;
}

/// Default validator function for identifiers.
/// Check that the identifier comprises only valid characters:
/// - Alphanumeric
/// - `.`, `_`, `+`, `-`, `#`
/// - `[`, `]`, `<`, `>`
/// and be of a given min and max
/// length.
pub fn validate_identifier(id: &str, min: usize, max: usize) -> Result<(), ValidationError> {
    assert!(max >= min);

    // Check identifier is not empty
    if id.is_empty() {
        bail!(ValidationKind::empty());
    }

    // Check identifier does not contain path separators
    if id.contains(PATH_SEPARATOR) {
        bail!(ValidationKind::contains_separator(id.to_string()));
    }

    // Check identifier length is between given min/max
    if id.len() < min || id.len() > max {
        bail!(ValidationKind::invalid_length(
            id.to_string(),
            id.len(),
            min,
            max
        ));
    }

    // Check that the identifier comprises only valid characters:
    // - Alphanumeric
    // - `.`, `_`, `+`, `-`, `#`
    // - `[`, `]`, `<`, `>`
    if !id
        .chars()
        .all(|c| c.is_alphanumeric() || VALID_SPECIAL_CHARS.contains(c))
    {
        bail!(ValidationKind::invalid_character(id.to_string()));
    }

    // All good!
    Ok(())
}
