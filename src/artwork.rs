use anyhow::{bail, Context, Result};
use std::{
    fs::{self, File},
    path::Path,
};

const ARTWORKS_NAME_TO_DESTINATION_NAME: [(&str, &str); 4] = [
    ("library_hero.jpg", "_hero.jpg"),
    ("logo.png", "_logo.png"),
    ("library_600x900.jpg", "p.jpg"),
    ("capsule_616x353.jpg", ".jpg"),
];

const STEAM_CDN: &str = "https://cdn.cloudflare.steamstatic.com/steam/apps/";

pub fn fetch_artworks(appid: u32, destination_folder: &Path) -> Result<()> {
    for (file_name, _) in ARTWORKS_NAME_TO_DESTINATION_NAME {
        let file_path = destination_folder.join(file_name);
        let mut dest_file = File::create(file_path.clone()).with_context(|| {
            format!(
                "Cannot create artwork image file at '{}'",
                file_path.to_str().unwrap()
            )
        })?;

        let mut resp = reqwest::blocking::get(format!("{STEAM_CDN}{appid}/{file_name}"))
            .with_context(|| format!("Cannot download artwork '{file_name}'"))?;
        if !resp.status().is_success() {
            bail!(
                "Http status code of artwork '{file_name}' is {}",
                resp.status()
            )
        }
        resp.copy_to(&mut dest_file)
            .with_context(|| "Couldn't copy response to already created image file")?;
    }

    Ok(())
}

pub fn move_artworks(input_folder: &Path, steam_grid_folder: &Path, new_appid: u32) -> Result<()> {
    assert!(steam_grid_folder.is_dir());

    for (cdn_name, standard_name) in ARTWORKS_NAME_TO_DESTINATION_NAME {
        let new_name = format!("{new_appid}{standard_name}");
        fs::rename(
            input_folder.join(cdn_name),
            steam_grid_folder.join(new_name),
        )
        .with_context(|| "Couldn't move artworks to steam grid folder")?;
    }

    Ok(())
}
