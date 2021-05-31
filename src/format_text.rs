use crate::configuration::Configuration;

pub fn format_text(file_text: &str, config: &Configuration) -> Result<String, taplo::parser::Error> {
    use taplo::{formatter, parser};

    let parser::Parse { green_node, errors } = parser::parse(file_text);

    match errors.first() {
        Some(e) => Err(e.clone()),
        None => Ok(formatter::format_green(green_node, config.clone())),
    }
}
