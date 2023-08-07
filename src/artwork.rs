use anyhow::Result;
use std::path::Path;
use crate::steam_api::download_artwork;

const ARTWORKS_NAME_TO_DESTINATION_NAME: [(&str, &str); 4] = [
    ("library_hero.jpg", "_hero.jpg"),
    ("logo.png", "_logo.png"),
    ("library_600x900.jpg", "p.jpg"),
    ("capsule_616x353.jpg", ".jpg"),
];

pub fn fetch_artworks(appid: u32, target_appid: u32, destination_folder: &Path) -> Result<()> {
    for (file_name, dest_name) in ARTWORKS_NAME_TO_DESTINATION_NAME {
        let dest = destination_folder.join(format!("{target_appid}{dest_name}"));
        download_artwork(appid, file_name, &dest)?;
    }

    Ok(())
}
