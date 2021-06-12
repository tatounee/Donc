use std::fmt;

use serde::{
    de::{Deserializer, Error, MapAccess, Visitor, IgnoredAny},
    Deserialize,
};

#[derive(Debug, Deserialize)]
pub struct Donation {
    pub level: usize,
    #[serde(rename = "maxLevel")]
    pub max_level: usize,
    pub name: String,
    pub village: String,
}


#[derive(Debug)]
pub struct Player {
    pub donations: u32,
    pub donable: Vec<Donation>,
}

impl<'de> Deserialize<'de> for Player {
    fn deserialize<D>(deserializer: D) -> Result<Player, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Donations,
            Donable,
            Ignore,
        }

        struct FieldVisitor;
        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Field;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match v {
                    "donations" => Ok(Field::Donations),
                    "troops" => Ok(Field::Donable),
                    "spells" => Ok(Field::Donable),
                    _ => Ok(Field::Ignore),
                }
            }
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                Deserializer::deserialize_identifier(deserializer, FieldVisitor)
            }
        }

        struct PlayerVisitor;
        impl<'de> Visitor<'de> for PlayerVisitor {
            type Value = Player;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Player")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                    A: MapAccess<'de>, {
                let mut donations: Option<u32> = None;
                let mut donable: Option<Vec<Donation>> = None;
                let mut donable_counter = 0;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Donations => {
                            if donations.is_some() {
                                return Err(Error::duplicate_field("donations"));
                            }
                            donations = Some(map.next_value()?)
                        }
                        Field::Donable => {
                            donable_counter += 1;
                            if donable_counter > 2 {
                                return Err(Error::duplicate_field("troops or spells"));
                            }
                            if let Some(ref mut vec) = donable {
                                vec.append(&mut map.next_value()?)
                            } else {
                                donable = Some(map.next_value()?)
                            }
                        }
                        Field::Ignore => {
                            let _ = map.next_value::<IgnoredAny>()?;
                        }
                    }
                }
                let donations = donations.ok_or_else(|| Error::missing_field("donations"))?;
                let donable = donable.ok_or_else(|| Error::missing_field("donations"))?;
                Ok(Player{donations, donable})
            }
        }
        
        const FIELDS: &'static [&'static str] = &["donations", "donable"];
        deserializer.deserialize_struct("Player", FIELDS, PlayerVisitor)
    }
}
