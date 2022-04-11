use crate::basic_profile::BasicProfile;
use crate::twitter::Twitter;

pub enum Schema {
    BasicProfile(BasicProfile),
    Twitter(Twitter)
}
