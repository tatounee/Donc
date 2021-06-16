mod clan;
mod donation;
mod error;
mod output;
mod player;

use std::env;

use dotenv::dotenv;
use pbr::ProgressBar;
use reqwest::{
    header::{HeaderMap, ACCEPT, AUTHORIZATION},
    Client,
};
use tokio::sync::mpsc;

use clan::Clan;
use error::Error;
use output::generate_csv;

use crate::player::Player;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let coc_token = env::vars()
        .find(|(k, _)| k == "COC_KEY_TOKEN")
        .ok_or(Error::NoApiTokenProvided)?
        .1;

    let clan_tag = env::args()
        .nth(1)
        .ok_or(Error::NoClanTagProvided)?
        .replace("#", "");

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
        .send()
        .await?
        .json::<Clan>()
        .await?;

    let mut pb = ProgressBar::new(clan.members.len() as u64 + 1);
    pb.message("Getting Player info ");

    let (tx, mut rx) = mpsc::channel(50);

    let mut players_handle = vec![];
    for member in clan.members {
        let sender = tx.clone();
        let client = client.clone();
        let header = header.clone();
        players_handle.push(tokio::spawn(async move {
            sender
                .send(
                    client
                        .get(format!(
                            "https://api.clashofclans.com/v1/players/{}",
                            member.tag.replace("#", "%23")
                        ))
                        .headers(header)
                        .send()
                        .await,
                )
                .await
        }))
    }

    drop(tx);

    let mut players = vec![];
    while let Some(resp) = rx.recv().await {
        players.push(resp?.json::<Player>().await?);
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
