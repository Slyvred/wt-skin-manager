use crate::api::structures::Skin;
use crate::backend::config::{Config, InstalledSkin};
use dioxus::prelude::*;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use zip::result::ZipError;
use zip::ZipArchive;

pub async fn install_skin(
    skin_signal: ReadSignal<Skin>,
    mut config_signal: Signal<Config>,
) -> Result<String, String> {
    let skin = skin_signal.read();
    let mut config = config_signal.read().clone();

    let resp = reqwest::get(&skin.file.link)
        .await
        .map_err(|e| e.to_string())?;
    let content = resp.bytes().await.map_err(|e| e.to_string())?;

    let str_path = format!("{}/{}", config.game_dir, skin.file.name);
    dbg!("Str path: {}", &str_path);
    let path = Path::new(&str_path);

    let mut out = File::create(&str_path).map_err(|e| e.to_string())?;

    let mut content_cursor = std::io::Cursor::new(content);
    io::copy(&mut content_cursor, &mut out).map_err(|e| e.to_string())?;

    let mut archive = File::open(&str_path)
        .map_err(ZipError::from)
        .and_then(ZipArchive::new)
        .map_err(|e| e.to_string())?;

    let mut needs_new_folder = false;
    let mut root_folder: Option<String> = None;

    for file_name in archive.file_names() {
        let mut parts = file_name.split('/');
        let first_part = parts.next().unwrap_or("");

        let has_subfolder = parts.next().is_some();

        if !has_subfolder && !file_name.ends_with('/') {
            needs_new_folder = true;
            break;
        }

        if let Some(ref root) = root_folder {
            if root != first_part {
                needs_new_folder = true;
                break;
            }
        } else {
            root_folder = Some(first_part.to_string());
        }
    }

    let final_extract_path = if needs_new_folder {
        let folder_name = skin.file.name.trim_end_matches(".zip");
        let new_dir_path = format!("{}/{}", config.game_dir, folder_name);

        std::fs::create_dir_all(&new_dir_path).map_err(|e| e.to_string())?;
        let _ = dbg!("No root folder in archive, creating one: {}", folder_name);
        new_dir_path
    } else {
        let _ = dbg!("Clean archive, proceeding to extraction");
        config.game_dir.to_string()
    };

    let extract_path_cpy = final_extract_path.clone();
    let extract_result = tokio::task::spawn_blocking(move || {
        archive
            .extract(&extract_path_cpy)
            .map_err(|e| format!("Failed to extract archive: {e}"))
    })
    .await
    .map_err(|_| "Thread pool error".to_string())?;

    extract_result?;

    let skin_path: String;
    if final_extract_path.ends_with("UserSkins") {
        skin_path = format!(
            "{}/{}",
            config.game_dir,
            skin.file.name.trim_end_matches(".zip")
        );
    } else {
        skin_path = final_extract_path;
    }

    let skin = InstalledSkin {
        lang_group: skin.lang_group,
        path: PathBuf::from(skin_path),
    };

    // config = config_signal.read().clone();
    config.installed_skins.push(skin);
    let _ = config.save();
    config_signal.set(config);

    match std::fs::remove_file(path) {
        Ok(_) => Ok("Skin installed successfully".to_string()),
        Err(e) => Ok(format!(
            "Skin installed successfully but unable to delete original archive, you may have to do it yourself: {}",
            e
        )),
    }
}
