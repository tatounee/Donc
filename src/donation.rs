use serde::Deserialize;

const fn default_false() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct Donation {
    pub level: usize,
    #[serde(rename = "maxLevel")]
    pub max_level: usize,
    pub name: String,
    pub village: String,
    #[serde(rename = "superTroopIsActive")]
    #[serde(default = "default_false")]
    pub super_troop_is_active: bool,
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
            "Dragon rider" => 15,

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
