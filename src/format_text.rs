use crate::configuration::Configuration;

pub fn format_text(file_text: &str, config: &Configuration) -> Result<String, String> {
    use taplo::{formatter, parser};

    let parser::Parse { green_node, errors } = parser::parse(file_text);
    if let Some(e) = errors.first() {
        return Err(e.message.clone());
    }
    Ok(formatter::format_green(green_node, config.clone()))
}
