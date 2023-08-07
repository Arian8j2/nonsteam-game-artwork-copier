mod game;

use anyhow::{bail, Context, Result};
use game::{parse_games_shortcut, Game};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Account {
    pub nonsteam_games: Vec<Game>,
    pub grid_folder_path: PathBuf,
}

pub fn parse_accounts(accounts_paths: Vec<PathBuf>) -> Result<Vec<Account>> {
    accounts_paths
        .into_iter()
        .map(|acc_path| acc_path.join("config").join("shortcuts.vdf"))
        .filter(|shortcut_path| shortcut_path.is_file())
        .map(|shortcut_path| {
            let shortcut_path_str = shortcut_path.to_string_lossy();
            let shortcut_content = fs::read(&shortcut_path).with_context(|| {
                format!("Couldn't read shortcut file at '{shortcut_path_str}'",)
            })?;

            let nonsteam_games = parse_games_shortcut(shortcut_content).with_context(|| {
                format!("Couldn't parse shortcut.vdf file located at '{shortcut_path_str}'")
            })?;

            let mut config_path = shortcut_path.clone();
            config_path.pop();

            let grid_folder_path = config_path.join("grid");
            if !grid_folder_path.is_dir() {
                bail!(
                    "Grid folder doesn't exists at '{}'",
                    grid_folder_path.to_str().unwrap()
                )
            }

            Ok(Account {
                nonsteam_games,
                grid_folder_path,
            })
        })
        .collect::<Result<Vec<Account>>>()
}

pub fn get_accounts_userdata_paths(steam_folder_path: &Path) -> Result<Vec<PathBuf>> {
    if !steam_folder_path.is_dir() {
        bail!(
            "Couldn't find steam files under '{}'",
            steam_folder_path.to_string_lossy()
        );
    }

    fs::read_dir(steam_folder_path.join("userdata"))
        .with_context(|| "Couldn't access files under userdata directory")?
        .map(|acc| {
            let acc =
                acc.with_context(|| "Faced error while trying to read from userdata directory")?;
            Ok(acc.path())
        })
        .collect::<Result<Vec<PathBuf>>>()
}
