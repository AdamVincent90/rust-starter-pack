use handlebars::handlebars_helper;
use rust_starter_pack::lib::logger::logger::Logger;
use serde_json;
use std::env;
use std::error::Error;
use std::fs::{create_dir, write};
use std::path::PathBuf;
use std::process::Command;

use super::client::create_client;
use super::store::create_store;

// ! This is still programmed in, lots of work to make this clean and less repetitive, and ofcourse, improved.

// We define our handlebars options here for more functionality in templates. (This should be moved)
handlebars_helper!(upper: |str: String| str[0..1].to_uppercase() + &str[1..]);

// Consts that represent the available options and paths.
const ALLOWED_OPTIONS: [&str; 2] = ["store", "client"];
const BASE_CORE_PATH: &str = "/src/core/";

// fn create_core() generates a new core entity using the pre-defined handlebars templates.
pub fn create_core(
    log: &Logger,
    command: &str,
    name: &str,
    opts: &[String],
) -> Result<(), Box<dyn Error>> {
    // Log the message
    let message = format!("processing {} with name {}", command, name);
    log.info_w(&message, Some("Lumber Create Core"));

    // we should route logic based on options provided.
    match extract_options(opts) {
        Some("store") => {
            log.info_w("found db option", Some("Lumber Create Core"));
            // Create core with store
            if let Err(err) = render_core(name, "core_mod_store", "core_with_store") {
                return Err(err);
            }
            if let Err(err) = create_store(log, command, name) {
                return Err(err);
            }
        }
        Some("client") => {
            log.info_w("found grpc option", Some("Lumber Create Core"));
            // Create core with client
            if let Err(err) = render_core(name, "core_mod_client", "core_with_client") {
                return Err(err);
            }
            if let Err(err) = create_client(log, command, name) {
                return Err(err);
            }
        }
        Some("all") => {
            log.info_w("found db and grpc options", Some("Lumber Create Core"));
            // Create core with client and store
            if let Err(err) = render_core(name, "core_mod_all", "core_with_all") {
                return Err(err);
            }
            if let Err(err) = create_client(log, command, name) {
                return Err(err);
            }
            if let Err(err) = create_store(log, command, name) {
                return Err(err);
            }
        }
        Some(&_) => {
            // Log invalid error
            log.error_w(
                "no valid options received, skipping generation.",
                Some("Lumber Create Core"),
            );
            return Ok(());
        }
        None => {
            log.info_w("found no additional options", Some("Lumber Create Core"));
            // No options, so do only the base logic.
            if let Err(err) = render_core(name, "core_mod_base", "core_base") {
                return Err(err);
            }
        }
    }

    // We create and spawn a new command to format our project.
    let mut formatter = Command::new("cargo");
    let formatter = formatter.arg("fmt");

    let result = match formatter.spawn() {
        Ok(result) => result,
        Err(err) => return Err(Box::new(err)),
    };

    let output = match result.wait_with_output() {
        Ok(result) => result,
        Err(err) => return Err(Box::new(err)),
    };

    log.info_w(
        format!("cargo format finished, status {}", output.status).as_str(),
        Some("Lumber Create Core"),
    );

    log.warn_w(
        "remember to register your new module in lib.rs!",
        Some("Lumber Create Core"),
    );

    Ok(())
}

// fn extract_options matches the options to route to the correct path, handling any potential logic from within.
// This function should and will contain additional validation and logic so we know everything is correct when routing
// To the correct templates.
fn extract_options(opts: &[String]) -> Option<&str> {
    match opts.len() {
        0 => None,
        2 => match opts[0].as_str() {
            "store" => {
                return Some("store");
            }
            "client" => {
                return Some("client");
            }
            _ => None,
        },
        4 => {
            if ALLOWED_OPTIONS.contains(&opts[0].as_str())
                && ALLOWED_OPTIONS.contains(&opts[2].as_str())
            {
                return Some("all");
            }
            None
        }
        _ => None,
    }
}

fn render_core(name: &str, core_mod_name: &str, core_name: &str) -> Result<(), Box<dyn Error>> {
    // Create a handlebars registry to use templates.
    let mut loader = handlebars::Handlebars::new();

    loader.register_helper("upper", Box::new(upper));

    // Define the absolute path.
    let abs_path = PathBuf::from(match env::current_dir() {
        Ok(abs_path) => abs_path,
        Err(err) => {
            return Err(Box::new(err));
        }
    });
    let abs_path = match abs_path.to_str() {
        Some(abs_path) => abs_path,
        None => return Err("could not convert absolute path to string".into()),
    };

    // Make sure we get the correct template paths
    let core_template_path = format!(
        "{}/src/app/tools/lumber/templates/core/{}.hbs",
        abs_path, core_name
    );

    let core_mod_path = format!(
        "{}/src/app/tools/lumber/templates/mods/{}.hbs",
        abs_path, core_mod_name,
    );

    let core_target_path = format!("{}{}{}", abs_path, BASE_CORE_PATH, name);

    // Get all relevant templates

    match loader.register_template_file(core_name, core_template_path) {
        Ok(loader) => loader,
        Err(err) => {
            return Err(Box::new(err));
        }
    }

    match loader.register_template_file(core_mod_name, core_mod_path) {
        Ok(loader) => loader,
        Err(err) => {
            return Err(Box::new(err));
        }
    }

    let data = serde_json::json!({
     "name": name,
    });

    if let Err(err) = create_dir(&core_target_path) {
        return Err(Box::new(err));
    }

    // Now generate the files, shadowing template is fine.
    let template = match loader.render(core_name, &data) {
        Ok(template) => template,
        Err(err) => return Err(Box::new(err)),
    };

    if let Err(err) = write(format!("{}/{}.rs", core_target_path, name), &template) {
        return Err(Box::new(err));
    }

    let template = match loader.render(core_mod_name, &data) {
        Ok(template) => template,
        Err(err) => return Err(Box::new(err)),
    };

    if let Err(err) = write(format!("{}/mod.rs", core_target_path), &template) {
        return Err(Box::new(err));
    }

    Ok(())
}
