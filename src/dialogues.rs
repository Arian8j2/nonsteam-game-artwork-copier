use super::account::Account;
use anyhow::{Context, Result};
use dialoguer::{console::style, theme::ColorfulTheme, Input, Select};
use lazy_static::lazy_static;

lazy_static! {
    static ref THEME: ColorfulTheme = {
        let mut theme = ColorfulTheme::default();
        theme.success_prefix = style("✓".to_string()).for_stderr().green();
        theme.checked_item_prefix = style("✓".to_string()).for_stderr().green();
        theme.unchecked_item_prefix = style("✓".to_string()).for_stderr().black();
        theme
    };
}

pub fn choose_nonsteam_game(accounts: &Vec<Account>) -> Result<(&Account, u32)> {
    let games = accounts
        .iter()
        .map(|a| {
            a.nonsteam_games
                .iter()
                .map(|game| game.name.as_str())
                .collect::<Vec<&str>>()
        })
        .flatten()
        .collect::<Vec<&str>>();

    let selected_game_index = Select::with_theme(&*THEME)
        .with_prompt("Select non steam game")
        .items(&games)
        .default(0)
        .interact()
        .with_context(|| "Failed to interact with select dialogue")?;
    let selected_game = games[selected_game_index];

    let game_account = accounts
        .iter()
        .find(|acc| {
            acc.nonsteam_games
                .iter()
                .find(|game| game.name == selected_game)
                .is_some()
        })
        .unwrap();

    let game = game_account
        .nonsteam_games
        .iter()
        .find(|game| game.name == selected_game)
        .unwrap();

    Ok((game_account, game.appid))
}

pub fn choose_existed_steam_appid() -> Result<u32> {
    let appid: u32 = Input::with_theme(&*THEME)
        .with_prompt("Enter existed steam app id")
        .validate_with(|input: &String| -> Result<(), String> {
            input
                .parse::<u32>()
                .map_err(|_| "App id must be positive number")?;
            Ok(())
        })
        .interact()?
        .parse()
        .unwrap();

    Ok(appid)
}
