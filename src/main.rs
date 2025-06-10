mod account;
mod artwork;
mod dialogues;
mod steam_api;

use account::{game::Game, get_accounts_userdata_paths, parse_accounts};
use anyhow::{Context, Result};
use artwork::fetch_artworks;
use dialoguer::console::style;
use dialogues::{choose_game_from_list, choose_nonsteam_game, choose_steam_game_name};
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    static ref STEAM_PATH: PathBuf = {
        if cfg!(target_os = "windows") {
            // Default Steam install path on Windows
            PathBuf::from(r"C:\Program Files (x86)\Steam")
        } else {
            // Default Steam path on Linux/macOS
            let home_dir = dirs::home_dir().expect("Home directory not found");
            home_dir.join(".steam").join("steam")
        }
    };
}

fn main() -> Result<()> {
    let account_paths = get_accounts_userdata_paths(&STEAM_PATH)?;
    let accounts = parse_accounts(account_paths)?;

    let (selected_account, selected_nonsteam_game) = choose_nonsteam_game(&accounts)?;
    let selected_game = ask_for_original_steam_game()?;

    let text = format!(
        "Copying artwork of '{}' to '{}'",
        selected_game.name, selected_nonsteam_game.name
    );
    println!("{}", style(text).blue());

    let grid_folder = &selected_account.grid_folder_path;
    fetch_artworks(
        selected_game.appid,
        selected_nonsteam_game.appid,
        &grid_folder,
    )
    .with_context(|| "Couldn't fetch and place artworks in steam directory")?;

    println!("{}", style("Restart steam to see changes").green());
    Ok(())
}

fn ask_for_original_steam_game() -> Result<Game> {
    loop {
        let game_name = choose_steam_game_name()?;
        let found_games = steam_api::search_game(&game_name)?;
        if found_games.is_empty() {
            let err_msg = format!("Couldn't find any game that matches name '{game_name}'");
            println!("{}", style(err_msg).red());
            continue;
        };

        println!(
            "{}",
            style("Press q or ESC to search with different query").blue()
        );

        let Some(steam_game) = choose_game_from_list(found_games)? else {
            continue;
        };
        return Ok(steam_game);
    }
}
