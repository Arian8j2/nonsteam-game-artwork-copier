use crate::account::game::Game;
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_json::Value;

const STEAM_CDN: &str = "https://cdn.cloudflare.steamstatic.com/steam/apps/";
const STEAM_SEARCH_APP_API: &str = "https://steamcommunity.com/actions/SearchApps/";

pub async fn download_artwork(appid: u32, artwork_name: &str) -> Result<Vec<u8>> {
    let url = format!("{STEAM_CDN}{appid}/{artwork_name}");
    let resp = reqwest::get(url.clone())
        .await
        .with_context(|| format!("Cannot download artwork at url '{url}'"))?;

    if !resp.status().is_success() {
        bail!("Http status code of artwork '{url}' is {}", resp.status())
    }

    let bytes = resp
        .bytes()
        .await
        .with_context(|| "Couldn't get response bytes")?;
    Ok(bytes.to_vec())
}

#[derive(Deserialize)]
pub struct SteamApp {
    pub name: String,
    pub appid: String,
}

pub fn search_game(query: &str) -> Result<Vec<Game>> {
    let resp = reqwest::blocking::get(format!("{STEAM_SEARCH_APP_API}{}", query.trim()))
        .with_context(|| "Couldn't search steam apps throught api")?;
    if !resp.status().is_success() {
        bail!(
            "Http error when fetch steam search app api: {}",
            resp.status()
        )
    }

    let resp = resp.text()?;
    let result: Value = serde_json::from_str(&resp)
        .with_context(|| "Steam search app api responded with invalid Json")?;
    let result: Vec<SteamApp> = serde_json::from_value(result)
        .with_context(|| format!("Couldn't deserialize steam search app api response: {resp}"))?;

    Ok(result
        .into_iter()
        .map(|steam_app| Game {
            name: steam_app.name,
            appid: steam_app.appid.parse().unwrap(),
        })
        .collect())
}
