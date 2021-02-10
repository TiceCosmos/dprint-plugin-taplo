use dprint_core::configuration::{
    get_unknown_property_diagnostics, get_value, ConfigKeyMap, GlobalConfiguration, NewLineKind,
    ResolveConfigurationResult,
};

pub type Configuration = taplo::formatter::Options;

pub fn resolve_config(
    config: ConfigKeyMap,
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
            "\t".to_string()
        } else {
            vec![' '; global_config.indent_width.unwrap_or(4) as usize]
                .into_iter()
                .collect()
        },
        &mut diagnostics,
    );

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
