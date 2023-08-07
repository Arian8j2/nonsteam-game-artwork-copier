mod account;
mod artwork;

use account::{get_accounts_userdata_paths, parse_accounts};
use anyhow::Result;
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
    for account in accounts {
        println!("{account:?}");
    }

    Ok(())
}
