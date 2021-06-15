use std::collections::HashMap;
use std::fmt;

use serde::{
    de::{Deserializer, Error, IgnoredAny, MapAccess, Visitor},
    Deserialize,
};

use crate::donation::{Donation, DonationUtils};

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
                A: MapAccess<'de>,
            {
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

                let mut real_level = HashMap::new();
                let donable = donable.ok_or_else(|| Error::missing_field("donations"))?;
                for d in donable.iter() {
                    if d.village == "home" {
                        if let Some(st) = d.get_super_troop() {
                            real_level.insert(st, d.level);
                        }
                    }
                }
                let donable = donable
                    .into_iter()
                    .filter_map(|mut d| {
                        if d.is_super_troop() && !d.super_troop_is_active || d.village != "home" {
                            None
                        } else {
                            if d.super_troop_is_active {
                                d.level = real_level[&d.name];
                            }
                            Some(d)
                        }
                    })
                    .collect::<Vec<Donation>>();

                Ok(Player { donations, donable })
            }
        }

        const FIELDS: &[&str] = &["donations", "donable"];
        deserializer.deserialize_struct("Player", FIELDS, PlayerVisitor)
    }
}
