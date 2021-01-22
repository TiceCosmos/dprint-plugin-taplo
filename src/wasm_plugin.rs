use dprint_core::configuration::{
    get_unknown_property_diagnostics, get_value, ConfigKeyMap, ConfigurationDiagnostic,
    GlobalConfiguration, ResolveConfigurationResult,
};
use dprint_core::generate_plugin_code;
use std::collections::HashMap;
use std::path::PathBuf;

use super::configuration::Configuration;

fn resolve_config(
    config: HashMap<String, ConfigKeyMap>,
    global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
    // implement. For example...
    let mut config = config;
    let mut diagnostics = Vec::new();
    let line_width = get_value(
        &mut config,
        "line_width",
        global_config.line_width.unwrap_or(120),
        &mut diagnostics,
    );

    diagnostics.extend(get_unknown_property_diagnostics(config));

    ResolveConfigurationResult {
        config: Configuration { line_width },
        diagnostics,
    }
}

fn get_plugin_config_key() -> String {
    // return the JSON object key name used in the configuration file
    // ex. String::from("json")
}

fn get_plugin_file_extensions() -> Vec<String> {
    // return the file extensions this plugin will format
    // ex. vec![String::from("json")]
}

fn get_plugin_help_url() -> String {
    // return the help url of the plugin
    // ex. String::from("https://dprint.dev/plugins/json")
}

fn get_plugin_config_schema_url() -> String {
    // for now, return an empty string. Return a schema url once VSCode
    // supports $schema properties in descendant objects:
    // https://github.com/microsoft/vscode/issues/98443
    String::new()
}

fn get_plugin_license_text() -> String {
    std::str::from_utf8(include_bytes!("../LICENSE"))
        .unwrap()
        .into()
}

fn format_text(
    file_path: &Path,
    file_text: &str,
    config: &Configuration,
) -> Result<String, String> {
    // implement...
}

generate_plugin_code!();
