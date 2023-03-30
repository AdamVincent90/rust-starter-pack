use handlebars::handlebars_helper;
use serde_json;
use std::env;
use std::fmt::Error;
use std::fs::{create_dir, write};
use std::path::PathBuf;
use std::process::Command;

use ultimate_rust_service::foundation::logger::logger::Logger;

handlebars_helper!(upper: |str: String| str[0..1].to_uppercase() + &str[1..]);

pub fn create_core(log: &Logger, command: &str, name: &str) -> Result<(), Error> {
    let message = format!("processing {} with name {}", command, name);
    log.info_w(&message, Some(()));

    let mut loader = handlebars::Handlebars::new();

    let abs_path = PathBuf::from(env::current_dir().unwrap());
    let abs_path = abs_path.to_str().unwrap();

    let template_path = format!("{}/src/app/tools/lumber/templates/core.hbs", abs_path);
    let target_path = format!("{}/src/business/core/{}", abs_path, name);

    loader
        .register_template_file(name, template_path)
        .unwrap_or_else(|err| {
            return Err(err).unwrap();
        });

    loader.register_helper("upper", Box::new(upper));

    let data = serde_json::json!({
     "name": name,
    });

    let template = loader.render(name, &data).unwrap_or_else(|err| {
        return Err(err).unwrap();
    });

    create_dir(&target_path).unwrap_or_else(|err| {
        return Err(err).unwrap();
    });

    write(format!("{}/{}.rs", target_path, name), template)
        .unwrap_or_else(|err| return Err(err).unwrap());

    let mut formatter = Command::new("cargo");
    let formatter = formatter.arg("fmt");

    formatter
        .spawn()
        .unwrap_or_else(|err| return Err(err).unwrap());

    Ok(())
}
