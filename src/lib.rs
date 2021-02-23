pub mod configuration;
mod format_text;

#[cfg(all(target_arch = "wasm32", target_os = "unknown", feature = "wasm"))]
mod wasm_plugin;

#[cfg(all(target_arch = "wasm32", target_os = "unknown", feature = "wasm"))]
pub use wasm_plugin::*;
