use dprint_core::{
    configuration::{get_nullable_value, get_unknown_property_diagnostics, resolve_new_line_kind},
    configuration::{ConfigKeyMap, GlobalConfiguration, ResolveConfigurationResult},
};

pub type Configuration = taplo::formatter::Options;

pub fn resolve_config(
    key_map: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
    let mut config = Configuration::default();

    if let Some(x) = global_config.line_width {
        config.column_width = x as usize;
    }
    if let Some(x) = global_config.indent_width {
        config.indent_string = std::iter::repeat(' ').take(x as usize).collect();
    }
    if global_config.use_tabs == Some(true) {
        config.indent_string = "\t".into();
    }
    if let Some(x) = global_config.new_line_kind {
        config.crlf = resolve_new_line_kind("", x) == "\r\n";
    }

    let mut key_map = key_map;
    let mut diagnostics = Vec::new();

    if let Some(x) = get_nullable_value(&mut key_map, "align_entries", &mut diagnostics) {
        config.align_entries = x;
    }

    if let Some(x) = get_nullable_value(&mut key_map, "array_trailing_comma", &mut diagnostics) {
        config.array_trailing_comma = x;
    }
    if let Some(x) = get_nullable_value(&mut key_map, "array_auto_expand", &mut diagnostics) {
        config.array_auto_expand = x;
    }
    if let Some(x) = get_nullable_value(&mut key_map, "array_auto_collapse", &mut diagnostics) {
        config.array_auto_collapse = x;
    }

    if let Some(x) = get_nullable_value(&mut key_map, "compact_arrays", &mut diagnostics) {
        config.compact_arrays = x;
    }
    if let Some(x) = get_nullable_value(&mut key_map, "compact_inline_tables", &mut diagnostics) {
        config.compact_inline_tables = x;
    }

    if let Some(x) = get_nullable_value(&mut key_map, "column_width", &mut diagnostics) {
        config.column_width = x;
    }

    if let Some(x) = get_nullable_value(&mut key_map, "indent_tables", &mut diagnostics) {
        config.indent_tables = x;
    }
    if let Some(x) = get_nullable_value(&mut key_map, "indent_string", &mut diagnostics) {
        config.indent_string = x;
    }

    if let Some(x) = get_nullable_value(&mut key_map, "trailing_newline", &mut diagnostics) {
        config.trailing_newline = x;
    }
    if let Some(x) = get_nullable_value(&mut key_map, "reorder_keys", &mut diagnostics) {
        config.reorder_keys = x;
    }
    if let Some(x) = get_nullable_value(&mut key_map, "allowed_blank_lines", &mut diagnostics) {
        config.allowed_blank_lines = x;
    }
    if let Some(x) = get_nullable_value(&mut key_map, "crlf", &mut diagnostics) {
        config.crlf = x;
    }

    diagnostics.extend(get_unknown_property_diagnostics(key_map));

    ResolveConfigurationResult {
        diagnostics,
        config,
    }
}
