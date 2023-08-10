use super::account::{game::Game, Account};
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

pub fn choose_nonsteam_game(accounts: &Vec<Account>) -> Result<(&Account, &Game)> {
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

    Ok((game_account, game))
}

pub fn choose_steam_game_name() -> Result<String> {
    let name: String = Input::with_theme(&*THEME)
        .with_prompt("Search steam original games")
        .interact()?;

    Ok(name)
}

pub fn choose_game_from_list(mut games: Vec<Game>) -> Result<Option<Game>> {
    let names = games.iter().map(|g| g.name.as_str()).collect::<Vec<&str>>();
    let selected_game_index = Select::with_theme(&*THEME)
        .with_prompt("Select a game")
        .items(&names)
        .interact_opt()?;

    Ok(selected_game_index.map(|index| games.remove(index)))
}
