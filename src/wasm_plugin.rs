use super::{resolve_config, Configuration};
use std::path::Path;

fn get_plugin_config_key() -> String {
    crate::CONFIG_KEY.into()
}

fn get_plugin_file_extensions() -> Vec<String> {
    crate::FILE_EXTENSIONS
        .iter()
        .map(|x| x.to_string())
        .collect()
}

fn get_plugin_help_url() -> String {
    crate::HELP_URL.into()
}

fn get_plugin_config_schema_url() -> String {
    crate::CONFIG_SCHEMA_URL.into()
}

fn get_plugin_license_text() -> String {
    crate::LICENSE_TEXT.into()
}

fn format_text(
    _file_path: &Path,
    file_text: &str,
    config: &Configuration,
) -> Result<String, String> {
    super::format_text(file_text, config).map_err(|x| x.message)
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
dprint_core::generate_plugin_code!();
