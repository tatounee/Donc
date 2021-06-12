
use std::fs::create_dir;
use std::time::{UNIX_EPOCH, SystemTime};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use csv::Writer;

use crate::player::Player;
use crate::error::Error;

const SECS_IN_SEASON: u64 = 3600 * 24 * 28;
const ONE_RESET_SEASON: u64 = 1622433600;
const END_OF_SEASON_STARTING: u64 = 3600 * 24 * 5;

pub fn generate_csv<P>(path: P, players: &[Player]) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let donations = players
        .iter()
        .filter(|p| p.donations > 100 * !is_season_starting() as u32)
        .map(|p| &p.donable)
        .flatten()
        .filter(|d| d.village == "home")
        .collect::<Vec<_>>();

    let max_level = donations
        .iter()
        .map(|d| d.max_level)
        .max()
        .unwrap_or(0);

    let mut sheet = HashMap::new();

    for donation in donations {
        let lvls = sheet
            .entry(&donation.name)
            .or_insert(vec![0u8; max_level + 1]);
        //* I don't know how supertroop's level work, so for now it's a little broken
        lvls[max_level - donation.level] += 1;
        lvls[max_level] += 1;
    }

    let mut sheet = sheet.into_iter().collect::<Vec<_>>();
    sheet.sort_by_key(|don| don.0.to_u32());

    if !PathBuf::from("./output").is_dir() {
        create_dir("./output")?;
    }

    let mut wtr = Writer::from_path(path)?;

    wtr.write_record(generate_colums(max_level))?;
    for (name, lvls) in sheet {
        if name.is_pet() {
            continue;
        }
        let id = name.to_u32();
        if id % 100 == 1 && id > 1 {
            wtr.write_record(vec![""; max_level + 2])?;
        }
        wtr.write_field(name)?;
        wtr.write_record(lvls.iter().map(|lvl| if lvl == &0 { String::new() } else { lvl.to_string() }))?;
    }
    wtr.flush()?;

    Ok(())
}

#[inline]
fn generate_colums(max_level: usize) -> Vec<String> {
    let mut row = Vec::with_capacity(max_level + 2);
    row.push("DONATION".to_owned());
    row.append(
        &mut (1..=max_level)
            .rev()
            .map(|x| format!("LvL {}", x))
            .collect::<Vec<String>>(),
    );
    row.push("TOTAL".to_owned());
    row
}

#[inline]
fn is_season_starting() -> bool {
    (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - ONE_RESET_SEASON) % SECS_IN_SEASON < END_OF_SEASON_STARTING
}

trait DonationCmp {
    fn to_u32(&self) -> u32;
    fn is_pet(&self) -> bool;
    // fn is_super_troop(&self) -> bool;
}

impl DonationCmp for String {
    fn to_u32(&self) -> u32 {
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
            "Super Wizard" => 206,
            "Inferno Dragon" => 207,
            "Super Minion" => 208,
            "Super Valkyrie" => 209,
            "Super Witch" => 210,
            "Ice Hound" => 211,
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
            _ => u32::MAX
        }
    }

    #[inline]
    fn is_pet(&self) -> bool {
        let id = self.to_u32();
        id > 600 && id <= 700
    }

    // #[inline]
    // fn is_super_troop(&self) -> bool {
    //     let value = self.to_u32();
    //     value > 200 && value <= 300
    // }
}
