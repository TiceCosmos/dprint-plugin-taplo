use dprint_core::configuration::{
    get_unknown_property_diagnostics, get_value, ConfigKeyMap, ConfigurationDiagnostic,
    GlobalConfiguration, NewLineKind, ResolveConfigurationResult,
};
use dprint_core::generate_plugin_code;
use std::path::PathBuf;
use std::{collections::HashMap, todo};

use super::configuration::Configuration;

fn resolve_config(
    config: HashMap<String, ConfigKeyMap>,
    global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
    let mut config = config;
    let mut diagnostics = Vec::new();

    let align_entries = get_value(&mut config, "align_entries", true, &mut diagnostics);
    let array_trailing_comma =
        get_value(&mut config, "array_trailing_comma", true, &mut diagnostics);
    let array_auto_expand = get_value(&mut config, "array_auto_expand", true, &mut diagnostics);
    let array_auto_collapse = get_value(&mut config, "array_auto_collapse", true, &mut diagnostics);

    let compact_arrays = get_value(&mut config, "compact_arrays", true, &mut diagnostics);
    let compact_inline_tables =
        get_value(&mut config, "compact_inline_tables", true, &mut diagnostics);

    let column_width = get_value(
        &mut config,
        "column_width",
        global_config.line_width.unwrap_or(120) as usize,
        &mut diagnostics,
    );

    let indent_tables = get_value(&mut config, "indent_tables", true, &mut diagnostics);
    let indent_string = get_value(
        &mut config,
        "indent_string",
        if global_config.use_tabs.unwrap_or(false) {
            "\t"
        } else {
            let mut indent_str = String::new();
            for _ in 0..global_config.indent_width {
                indent_str.push(' ');
            }
        },
        &mut diagnostics,
    )
    .to_string();

    let trailing_newline = get_value(&mut config, "trailing_newline", true, &mut diagnostics);
    let reorder_keys = get_value(&mut config, "reorder_keys", true, &mut diagnostics);

    let allowed_blank_lines = get_value(&mut config, "allowed_blank_lines", 0, &mut diagnostics);

    let crlf = get_value(
        &mut config,
        "crlf",
        global_config.new_line_kind == Some(NewLineKind::CarriageReturnLineFeed),
        &mut diagnostics,
    );

    diagnostics.extend(get_unknown_property_diagnostics(config));

    ResolveConfigurationResult {
        config: Configuration {
            align_entries,
            array_trailing_comma,
            array_auto_expand,
            array_auto_collapse,
            compact_arrays,
            compact_inline_tables,
            column_width,
            indent_tables,
            indent_string,
            trailing_newline,
            reorder_keys,
            allowed_blank_lines,
            crlf,
        },
        diagnostics,
    }
}

fn get_plugin_config_key() -> String {
    String::from("taplo")
}

fn get_plugin_file_extensions() -> Vec<String> {
    vec![String::from("toml")]
}

fn get_plugin_help_url() -> String {
    String::from("https://taplo.tamasfe.dev/configuration/#formatting-options")
}

fn get_plugin_config_schema_url() -> String {
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
    todo!();
}

generate_plugin_code!();
