use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_json::Value;
use std::{fs::File, path::Path};

const STEAM_APP_DETAIL_API: &str = "https://store.steampowered.com/api/appdetails";
const STEAM_CDN: &str = "https://cdn.cloudflare.steamstatic.com/steam/apps/";

#[derive(Deserialize)]
pub struct SteamApp {
    pub name: String,
    pub steam_appid: u32,
}

#[derive(Deserialize)]
struct SteamAppDetailResponse {
    success: bool,
    data: Option<SteamApp>,
}

pub fn fetch_steamapp_info(appid: u32) -> Result<Option<SteamApp>> {
    let resp = reqwest::blocking::get(format!("{STEAM_APP_DETAIL_API}?appids={appid}"))
        .with_context(|| "Couldn't fetch steam app detail")?;

    if !resp.status().is_success() {
        bail!("Http error when fetch steam app detail: {}", resp.status())
    }

    let result: Value = serde_json::from_str(&resp.text()?)
        .with_context(|| "Steam fetch app detail api responded with invalid Json")?;
    let result = result.get(appid.to_string()).unwrap().clone();

    let result: SteamAppDetailResponse = serde_json::from_value(result)
        .with_context(|| "Couldn't deserialize steam app detail api response")?;

    if !result.success {
        Ok(None)
    } else {
        let data = result.data.unwrap();
        assert!(data.steam_appid == appid);
        Ok(Some(data))
    }
}

pub fn download_artwork(appid: u32, artwork_name: &str, destination_path: &Path) -> Result<()> {
    let mut dest_file = File::create(destination_path).with_context(|| {
        format!(
            "Cannot create artwork image file at '{}'",
            destination_path.to_str().unwrap()
        )
    })?;

    let url = format!("{STEAM_CDN}{appid}/{artwork_name}");
    let mut resp = reqwest::blocking::get(url.clone())
        .with_context(|| format!("Cannot download artwork at url '{url}'"))?;
    if !resp.status().is_success() {
        bail!("Http status code of artwork '{url}' is {}", resp.status())
    }
    resp.copy_to(&mut dest_file)
        .with_context(|| "Couldn't copy response to already created image file")?;

    Ok(())
}
