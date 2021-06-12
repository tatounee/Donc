mod clan;
mod error;
mod output;
mod player;

use std::env;

use pbr::ProgressBar;
use dotenv::dotenv;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, ACCEPT, AUTHORIZATION},
};

use clan::Clan;
use error::Error;
use output::generate_csv;

use crate::player::Player;

fn main() -> Result<(), Error> {
    dotenv().ok();
    let coc_token = env::vars()
        .find(|(k, _)| k == "COC_KEY_TOKEN")
        .ok_or_else(|| Error::NoApiTokenProvided)?
        .1;

    let clan_tag = env::args().nth(1).ok_or_else(|| Error::NoClanTagProvided)?.replace("#", "");

    let mut header = HeaderMap::new();
    header.insert(ACCEPT, "application/json".parse().unwrap());
    header.insert(
        AUTHORIZATION,
        format!("Bearer {}", coc_token).parse().unwrap(),
    );


    let client = Client::new();
    let clan = client
        .get(format!(
            "https://api.clashofclans.com/v1/clans/%23{}",
            clan_tag
        ))
        .headers(header.clone())
        .send()?
        .json::<Clan>()?;

    let mut pb = ProgressBar::new(clan.members.len() as u64 + 1);
    pb.message("Getting Player info ");

    let mut players = vec![];
    for member in clan.members {
        players.push(
            client
                .get(format!(
                    "https://api.clashofclans.com/v1/players/{}",
                    member.tag.replace("#", "%23")
                ))
                .headers(header.clone())
                .send()?
                .json::<Player>()?,
        );
        pb.inc();
    }

    pb.message("Exporting data ");

    let mut path_output = env::current_dir()?;
    path_output.push(format!("output\\{} [{}].csv", clan.name, clan_tag));

    generate_csv(&path_output, &players)?;

    pb.inc();
    
    println!("\nYou can find your data at {}", path_output.display());

    Ok(())
}
