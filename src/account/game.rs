use anyhow::{Context, Result};
use regex::bytes::Regex;

#[derive(Debug, Clone)]
pub struct Game {
    pub name: String,
    pub appid: u32,
}

pub fn parse_games_shortcut(content: Vec<u8>) -> Result<Vec<Game>> {
    let re = Regex::new(r"(?-u)appid\x00(.{4}).AppName\x00(.*?)\x00")?;
    let games = re
        .captures_iter(&content)
        .map(|captures| {
            let (_, [appid_bytes, name_bytes]) = captures.extract();

            let appid_bytes: [u8; 4] = appid_bytes[..4]
                .try_into()
                .with_context(|| "Couldn't convert appid to u32 little endian integer")?;
            let appid = u32::from_ne_bytes(appid_bytes);
            let name = String::from_utf8(name_bytes.to_vec())
                .with_context(|| "Cannot convert name into utf8")?;

            anyhow::Ok(Game { name, appid })
        })
        .collect::<Result<Vec<Game>>>()?;

    Ok(games)
}
