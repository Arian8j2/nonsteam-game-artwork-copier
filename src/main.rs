mod account;
mod artwork;
mod dialogues;
mod steam_api;

use account::{get_accounts_userdata_paths, parse_accounts};
use anyhow::{Context, Result};
use artwork::fetch_artworks;
use dialoguer::console::style;
use dialogues::{choose_existed_steam_appid, choose_nonsteam_game};
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    static ref STEAM_PATH: PathBuf = {
        let home_dir = dirs::home_dir().expect("Home directory not found");
        home_dir.join(".steam").join("steam")
    };
}

fn main() -> Result<()> {
    let account_paths = get_accounts_userdata_paths(&STEAM_PATH)?;
    let accounts = parse_accounts(account_paths)?;

    let (selected_account, selected_nonsteam_game) = choose_nonsteam_game(&accounts)?;

    let selected_game_appid = loop {
        let appid = choose_existed_steam_appid()?;
        let Some(game_info) = steam_api::fetch_steamapp_info(appid)? else {
            let err_msg = format!("Couldn't find any game that match steamapp id '{appid}'");
            println!("{}", style(err_msg).red());
            continue;
        };

        let info = style(format!("Changing artworks to '{}'", game_info.name));
        println!("{}", info.yellow());
        break game_info.steam_appid;
    };

    let grid_folder = &selected_account.grid_folder_path;
    fetch_artworks(selected_game_appid, selected_nonsteam_game, &grid_folder)
        .with_context(|| "Couldn't fetch and place artworks in steam directory")?;

    println!("{}", style("Restart steam to see changes").green());
    Ok(())
}
