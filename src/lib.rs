pub mod configuration;
mod format_text;

pub use configuration::{resolve_config, Configuration};
pub use format_text::format_text;

#[cfg(feature = "wasm")]
mod wasm_plugin;

#[cfg(feature = "wasm")]
pub use wasm_plugin::*;

#[cfg(feature = "process")]
mod process_plugin;

#[cfg(feature = "process")]
pub use process_plugin::*;

const CONFIG_KEY: &str = "taplo";
const FILE_EXTENSIONS: [&str; 1] = ["toml"];
const HELP_URL: &str = "https://taplo.tamasfe.dev/configuration/#formatting-options";
const CONFIG_SCHEMA_URL: &str = "";
const LICENSE_TEXT: &str = include_str!("../LICENSE");
