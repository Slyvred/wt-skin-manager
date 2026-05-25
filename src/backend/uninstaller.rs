use crate::api::structures::Skin;
use crate::backend::config::Config;
use dioxus::logger::tracing;
use dioxus::prelude::*;

pub async fn uninstall_skin(
    skin_signal: ReadSignal<Skin>,
    mut config_signal: Signal<Config>,
) -> Result<String, String> {
    let skin = skin_signal.read();
    let mut config = config_signal.write();

    let (idx, skin_to_remove) = config
        .installed_skins
        .iter()
        .enumerate()
        .find(|(_, x)| x.lang_group == skin.lang_group)
        .unwrap();

    tracing::debug!("Index: {}\nSkin {:?}", idx, skin_to_remove);

    let delete_res = std::fs::remove_dir_all(&skin_to_remove.path);

    config.installed_skins.remove(idx);

    let _ = config.save();

    match delete_res {
        Ok(_) => Ok("Skin uninstalled successfully".to_string()),
        Err(e) => Err(format!("Failed to uninstall skin: {}", e)),
    }
}
