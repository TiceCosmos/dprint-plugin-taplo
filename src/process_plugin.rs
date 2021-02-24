use super::Configuration;
use dprint_core::{
    configuration::{ConfigKeyMap, GlobalConfiguration, ResolveConfigurationResult},
    plugins::{process::ProcessPluginHandler, PluginInfo},
    types::ErrBox,
};
use std::path::Path;

pub struct MyProcessPluginHandler {}
impl Default for MyProcessPluginHandler {
    fn default() -> Self {
        Self {}
    }
}

impl ProcessPluginHandler<Configuration> for MyProcessPluginHandler {
    fn get_plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: String::from(env!("CARGO_PKG_NAME")),
            version: String::from(env!("CARGO_PKG_VERSION")),
            config_key: crate::CONFIG_KEY.into(),
            file_extensions: crate::FILE_EXTENSIONS
                .iter()
                .map(|x| x.to_string())
                .collect(),
            help_url: crate::HELP_URL.into(),
            config_schema_url: crate::CONFIG_SCHEMA_URL.into(),
        }
    }

    fn get_license_text(&self) -> &str {
        crate::LICENSE_TEXT
    }

    fn resolve_config(
        &self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Configuration> {
        super::resolve_config(config, global_config)
    }

    fn format_text<'a>(
        &'a self,
        _file_path: &Path,
        file_text: &str,
        config: &Configuration,
        _format_with_host: Box<
            dyn FnMut(&Path, String, &ConfigKeyMap) -> Result<String, ErrBox> + 'a,
        >,
    ) -> Result<String, ErrBox> {
        Ok(super::format_text(file_text, config)?)
    }
}
