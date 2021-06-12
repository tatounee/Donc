
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Clan {
    pub name: String,
    #[serde(rename = "memberList")]
    pub members: Vec<Member>
}

#[derive(Deserialize, Debug)]
pub struct Member {
    pub tag: String,
}
