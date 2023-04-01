use handlebars::handlebars_helper;
use serde_json;
use std::env;
use std::fmt::Error;
use std::fs::{create_dir, write};
use std::path::PathBuf;
use std::process::Command;

use ultimate_rust_service::foundation::logger::logger::Logger;

handlebars_helper!(upper: |str: String| str[0..1].to_uppercase() + &str[1..]);

const ALLOWED_OPTIONS: [&str; 2] = ["store", "client"];
const ALLOWED_STORE_OPTIONS: [&str; 1] = ["db"];
const ALLOWED_CLIENT_OPTIONS: [&str; 1] = ["grpc"];

const BASE_CORE_PATH: &str = "/src/business/core/";

pub fn create_core(log: &Logger, command: &str, name: &str, opts: &[String]) -> Result<(), Error> {
    // I believe its better to do core command only validation here..

    if opts.len() > 0 {
        if !ALLOWED_OPTIONS.contains(&opts[0].as_str()) {
            log.error_w(
                "no valid options, expecting store or client, but received",
                Some(&opts[0].as_str()),
            );
            return Err(()).unwrap();
        }
    }

    let message = format!("processing {} with name {}", command, name);
    log.info_w(&message, Some(()));

    let mut loader = handlebars::Handlebars::new();

    let abs_path = PathBuf::from(env::current_dir().unwrap());
    let abs_path = abs_path.to_str().unwrap();

    let template_path = format!("{}/src/app/tools/lumber/templates/core.hbs", abs_path);
    let target_path = format!("{}{}{}", abs_path, BASE_CORE_PATH, name);

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

    write(format!("{}/{}.rs", target_path, name), &template)
        .unwrap_or_else(|err| return Err(err).unwrap());

    // Render options. (Engineering needs to be done here)

    if opts.len() > 0 {
        match opts[0].as_str() {
            "store" => {
                let parts = opts[1].split(",");
                for i in parts {
                    match i {
                        "db" => {
                            let path = format!("{}/stores/{}_db", target_path, name);
                            create_dir(&path).unwrap_or_else(|err| {
                                return Err(err).unwrap();
                            });

                            loader
                                .register_template_file(
                                    "db",
                                    format!("{}/src/app/tools/lumber/templates/db.hbs", abs_path),
                                )
                                .unwrap();

                            let template = loader.render("db", &data).unwrap_or_else(|err| {
                                return Err(err).unwrap();
                            });

                            write(format!("{}/{}_db.rs", path, name), template.clone())
                                .unwrap_or_else(|err| return Err(err).unwrap());
                        }
                        _ => log.warn_w(
                            "option is not valid for store skipping.. : invalid option =>",
                            Some(i),
                        ),
                    }
                }
            }
            "client" => {}
            _ => {}
        }
    }

    let mut formatter = Command::new("cargo");
    let formatter = formatter.arg("fmt");

    formatter
        .spawn()
        .unwrap_or_else(|err| return Err(err).unwrap());

    Ok(())
}
