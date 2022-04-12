use crate::schema::basic_profile::BasicProfile;
use crate::schema::twitter::Twitter;

pub enum Schema {
    BasicProfile(BasicProfile),
    Twitter(Twitter)
}
