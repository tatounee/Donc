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

pub trait DonationUtils {
    type Output;

    fn as_u32(&self) -> u32;
    fn is_pet(&self) -> bool;
    fn is_super_troop(&self) -> bool;
    fn get_super_troop(&self) -> Option<Self::Output>;
}

impl DonationUtils for String {
    type Output = String;

    fn as_u32(&self) -> u32 {
        match self.as_ref() {
            "Barbarian" => 1,
            "Archer" => 2,
            "Goblin" => 3,
            "Giant" => 4,
            "Wall Breaker" => 5,
            "Balloon" => 6,
            "Wizard" => 7,
            "Healer" => 8,
            "Dragon" => 9,
            "P.E.K.K.A" => 10,
            "Baby Dragon" => 11,
            "Miner" => 12,
            "Electro Dragon" => 13,
            "Yeti" => 14,

            "Minion" => 101,
            "Hog Rider" => 102,
            "Valkyrie" => 103,
            "Golem" => 104,
            "Witch" => 105,
            "Lava Hound" => 106,
            "Bowler" => 107,
            "Ice Golem" => 108,
            "Headhunter" => 109,

            "Super Barbarian" => 201,
            "Super Archer" => 202,
            "Super Giant" => 203,
            "Sneaky Goblin" => 204,
            "Super Wall Breaker" => 205,
            "Rocket Balloon" => 206,
            "Super Wizard" => 207,
            "Inferno Dragon" => 208,
            "Super Minion" => 209,
            "Super Valkyrie" => 210,
            "Super Witch" => 211,
            "Ice Hound" => 212,

            "Lightning Spell" => 301,
            "Healing Spell" => 302,
            "Rage Spell" => 303,
            "Jump Spell" => 304,
            "Freeze Spell" => 305,
            "Clone Spell" => 306,
            "Invisibility Spell" => 307,

            "Poison Spell" => 401,
            "Earthquake Spell" => 402,
            "Haste Spell" => 403,
            "Skeleton Spell" => 404,
            "Bat Spell" => 405,

            "Wall Wrecker" => 501,
            "Battle Blimp" => 502,
            "Stone Slammer" => 503,
            "Siege Barracks" => 504,
            "Log Launcher" => 505,

            "Mighty Yak" => 601,
            "Unicorn" => 602,
            "L.A.S.S.I" => 603,
            "Electro Owl" => 604,
            _ => u32::MAX,
        }
    }

    #[inline]
    fn is_pet(&self) -> bool {
        let id = self.as_u32();
        id > 600 && id <= 700
    }

    #[inline]
    fn is_super_troop(&self) -> bool {
        let id = self.as_u32();
        id > 200 && id <= 300
    }

    fn get_super_troop(&self) -> Option<Self::Output> {
        if self.as_u32() > 200 {
            None
        } else {
            match self.as_ref() {
                "Barbarian" => Some("Super Barbarian".to_owned()),
                "Archer" => Some("Super Archer".to_owned()),
                "Goblin" => Some("Sneaky Goblin".to_owned()),
                "Giant" => Some("Super Giant".to_owned()),
                "Wall Breaker" => Some("Super Wall Breaker".to_owned()),
                "Balloon" => Some("Rocket balloon".to_owned()),
                "Wizard" => Some("Super Wizard".to_owned()),
                "Baby Dragon" => Some("Inferno Dragon".to_owned()),
                "Minion" => Some("Super Minion".to_owned()),
                "Valkyrie" => Some("Super Valkyrie".to_owned()),
                "Witch" => Some("Super Witch".to_owned()),
                "Lava Hound" => Some("Ice Hound".to_owned()),

                _ => None,
            }
        }
    }
}

impl DonationUtils for Donation {
    type Output = String;

    #[inline]
    fn as_u32(&self) -> u32 {
        self.name.as_u32()
    }

    #[inline]
    fn is_pet(&self) -> bool {
        self.name.is_pet()
    }

    #[inline]
    fn is_super_troop(&self) -> bool {
        self.name.is_super_troop()
    }

    #[inline]
    fn get_super_troop(&self) -> Option<Self::Output> {
        self.name.get_super_troop()
    }
}
