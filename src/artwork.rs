use crate::steam_api::download_artwork;
use anyhow::{Context, Result};
use std::{
    io::Cursor,
    path::{Path, PathBuf},
};
use tokio::{fs::File, task::JoinSet};

const ARTWORKS_NAME_TO_DESTINATION_NAME: [(&str, &str); 4] = [
    ("library_hero.jpg", "_hero.jpg"),
    ("logo.png", "_logo.png"),
    ("library_600x900.jpg", "p.jpg"),
    ("capsule_616x353.jpg", ".jpg"),
];

#[tokio::main(flavor = "current_thread")]
pub async fn fetch_artworks(
    appid: u32,
    target_appid: u32,
    destination_folder: &Path,
) -> Result<()> {
    let mut tasks = JoinSet::new();

    for (artwork_name, dest_name) in ARTWORKS_NAME_TO_DESTINATION_NAME {
        let dest_path = destination_folder.join(format!("{target_appid}{dest_name}"));
        tasks.spawn(fetch_single_artwork(appid, artwork_name, dest_path));
    }

    while let Some(result) = tasks.join_next().await {
        result
            .with_context(|| "Couldn't join download artwork task")?
            .with_context(|| "Downloading artwork failed")?;
    }
    Ok(())
}

async fn fetch_single_artwork(appid: u32, artwork_name: &str, dest_path: PathBuf) -> Result<()> {
    let bytes = download_artwork(appid, artwork_name).await?;

    let dest_path_str = dest_path.to_string_lossy();
    let mut dest_file = File::create(&dest_path)
        .await
        .with_context(|| format!("Cannot create artwork image file at '{dest_path_str}'",))?;

    let mut content = Cursor::new(bytes);
    tokio::io::copy(&mut content, &mut dest_file)
        .await
        .with_context(|| "Couldn't write to file at '{dest_path_str}'")?;
    Ok(())
}
