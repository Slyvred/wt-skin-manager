use crate::api::structures::Skin;
use crate::backend::config::{Config, InstalledSkin};
use dioxus::prelude::*;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use zip::result::ZipError;
use zip::ZipArchive;

async fn download_archive(url: &str, download_path: &str) -> Result<ZipArchive<File>, String> {
    let resp = reqwest::get(url).await.map_err(|e| e.to_string())?;
    let content = resp.bytes().await.map_err(|e| e.to_string())?;
    let path = Path::new(&download_path);

    let mut out = File::create(&path).map_err(|e| e.to_string())?;

    let mut content_cursor = std::io::Cursor::new(content);
    io::copy(&mut content_cursor, &mut out).map_err(|e| e.to_string())?;

    let archive = File::open(&path)
        .map_err(ZipError::from)
        .and_then(ZipArchive::new)
        .map_err(|e| e.to_string())?;

    return Ok(archive);
}

async fn extract_skin(
    mut archive: ZipArchive<File>,
    filename: &str,
    game_dir: &str,
) -> Result<PathBuf, String> {
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
        let folder_name = filename.trim_end_matches(".zip");
        let new_dir_path = format!("{}/{}", game_dir, folder_name);

        std::fs::create_dir_all(&new_dir_path).map_err(|e| e.to_string())?;
        let _ = dbg!("No root folder in archive, creating one: {}", folder_name);
        new_dir_path
    } else {
        let _ = dbg!("Clean archive, proceeding to extraction");
        game_dir.to_string()
    };

    let extract_path_cpy = final_extract_path.clone();
    let _ = tokio::task::spawn_blocking(move || {
        archive
            .extract(&extract_path_cpy)
            .map_err(|e| format!("Failed to extract archive: {e}"))
    })
    .await
    .map_err(|_| "Thread pool error".to_string())?;

    // If the archive doesn't have a root folder we extract it in the folder we created above
    if needs_new_folder {
        Ok(PathBuf::from(final_extract_path))
    }
    // But if it's well formatted the extracted folder will be the root folder of the archive.
    // So we need to return that instead
    else {
        let zip_root = root_folder.unwrap_or_default();
        let skin_dir = PathBuf::from(game_dir).join(zip_root);
        Ok(skin_dir)
    }
}

pub async fn install_skin(skin: Skin, mut config_signal: Signal<Config>) -> Result<String, String> {
    let game_dir = config_signal.read().game_dir.clone();
    let archive_path = format!("{}/{}", &game_dir, &skin.file.name);
    let archive = download_archive(&skin.file.link, &archive_path).await?;
    let skin_path = extract_skin(archive, &skin.file.name, &game_dir).await?;

    let skin = InstalledSkin {
        lang_group: skin.lang_group,
        path: skin_path.clone(),
    };

    {
        let mut config = config_signal.write();
        config.installed_skins.push(skin);
        let _ = config.save();
    }

    match std::fs::remove_file(&archive_path) {
        Ok(_) => Ok("Skin installed successfully".to_string()),
        Err(e) => Ok(format!(
            "Skin installed successfully but unable to delete original archive, you may have to do it yourself: {}",
            e
        )),
    }
}
