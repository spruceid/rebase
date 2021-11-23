mod discord;
mod twitter;

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

// TODO: Proper cased names and a serialization trick.
#[derive(Deserialize, Serialize)]
pub enum SignerType {
    #[serde(rename = "eth")]
    Eth,
    #[serde(rename = "tz")]
    Tz,
}

impl Display for SignerType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            SignerType::Eth => write!(f, "eth"),
            SignerType::Tz => write!(f, "tz"),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Location {
    DiscordVerification(discord::Location),
    TwitterVerification(twitter::Location),
}

#[derive(Deserialize, Serialize)]
pub struct PublicClaim {
    pub poster_id: String,
    pub signer_id: String,
    pub version: u64,
    pub location: Location,
}

#[derive(Deserialize, Serialize)]
pub struct SignedClaim {
    credential_subject_id: String,
    data: PublicClaim,
    full: String,
    signed: String,
    signer_type: String,
    unsigned: String,
}

impl SignedClaim {
    // Move to this_error
    pub fn sanity_check(&self) -> Result<(), String> {
        if format!("{}{}", self.unsigned, self.signed) != self.full {
            return Err("SignedClaim full did not match the concatenation of unsigned and signed".to_owned());
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
