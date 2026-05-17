use core::arch;
use std::error::Error;
use std::fs::{self, File};
use std::io;
use std::path::Path;
use tokio;
use zip::read::root_dir_common_filter;
use zip::result::ZipError;
use zip::ZipArchive;

pub async fn install_skin(file_link: &str, filename: &str) -> Result<String, String> {
    //-> Result<(), Box<dyn Error>> {
    let resp = reqwest::get(file_link).await.expect("Failed to download");
    let content = resp.bytes().await.expect("Failed to retrieve content");

    let game_dir = "/home/slyvred/Documents/WarThunder/UserSkins";
    let str_path = format!("{}/{}", game_dir, filename);
    let path = Path::new(&str_path);

    let mut out = File::create(&str_path).expect("failed to create file");
    io::copy(&mut content.iter().as_slice(), &mut out).expect("failed to copy content");

    let mut archive = File::open(&str_path)
        .map_err(ZipError::from)
        .and_then(ZipArchive::new)
        .expect("Failed to open archive");

    for file in archive.file_names() {
        println!("{}", file);
    }

    let result = tokio::task::spawn_blocking(move || match archive.extract(game_dir) {
        Ok(_) => println!("OK"),
        Err(_) => eprintln!("ERROR"),
    })
    .await
    .map_err(|_| "Thread pool error".to_string())
    .map_err(|e| format!("Failed to parse extract archive: {e}"));

    // If the camo was extracted successfully we can remove the original zip file
    match result {
        Ok(_) => match std::fs::remove_file(path) {
            Ok(_) => Ok("Skin installed successfully !".to_string()),
            Err(e) => Ok(format!(
                "Skin installed successfully but failed to remove original archive: {}",
                e
            )),
        },
        Err(e) => return Err(e),
    }
}
