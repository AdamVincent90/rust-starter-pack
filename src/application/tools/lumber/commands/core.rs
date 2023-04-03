use handlebars::handlebars_helper;
use rust_starter_pack::dependency::logger::logger::Logger;
use serde_json;
use std::env;
use std::fmt::Error;
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
const BASE_CORE_PATH: &str = "/src/business/core/";

// fn create_core() generates a new core entity using the pre-defined handlebars templates.
pub fn create_core(log: &Logger, command: &str, name: &str, opts: &[String]) -> Result<(), Error> {
    // Log the message
    let message = format!("processing {} with name {}", command, name);
    log.info_w(&message, Some(()));

    // we should route logic based on options provided.
    match extract_options(opts) {
        Some("store") => {
            log.info_w("found db option", Some(()));
            // Create core with store
            if let Err(err) = render_core(name, "core_mod_store", "core_with_store") {
                return Err(err).unwrap();
            }
            if let Err(err) = create_store(log, command, name) {
                return Err(err).unwrap();
            }
        }
        Some("client") => {
            log.info_w("found grpc option", Some(()));
            // Create core with client
            if let Err(err) = render_core(name, "core_mod_client", "core_with_client") {
                return Err(err).unwrap();
            }
            if let Err(err) = create_client(log, command, name) {
                return Err(err).unwrap();
            }
        }
        Some("all") => {
            log.info_w("found db and grpc options", Some(()));
            // Create core with client and store
            if let Err(err) = render_core(name, "core_mod_all", "core_with_all") {
                return Err(err).unwrap();
            }
            if let Err(err) = create_client(log, command, name) {
                return Err(err).unwrap();
            }
            if let Err(err) = create_store(log, command, name) {
                return Err(err).unwrap();
            }
        }
        Some(&_) => {
            // Log invalid error
            log.error_w("invalid option received", Some(()));
            return Err(()).unwrap();
        }
        None => {
            log.info_w("found no additional options", Some(()));
            // No options, so do only the base logic.
            if let Err(err) = render_core(name, "core_mod_base", "core_base") {
                return Err(err).unwrap();
            }
        }
    }

    // We create and spawn a new command to format our project.
    let mut formatter = Command::new("cargo");
    let formatter = formatter.arg("fmt");

    let result = formatter.spawn().unwrap_or_else(|err| {
        log.warn_w(
            "generation completed, but unable to format files.",
            Some(&err),
        );
        return Err(err).unwrap();
    });

    let output = result.wait_with_output().unwrap();

    log.info_w("cargo format finished", Some(output));

    log.warn_w("remember to register your new module in lib.rs!", Some(()));

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

fn render_core(name: &str, core_mod_name: &str, core_name: &str) -> Result<(), Error> {
    // Create a handlebars registry to use templates.
    let mut loader = handlebars::Handlebars::new();

    loader.register_helper("upper", Box::new(upper));

    // Define the absolute path.
    let abs_path = PathBuf::from(env::current_dir().unwrap());
    let abs_path = abs_path.to_str().unwrap();

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

    loader
        .register_template_file(core_name, core_template_path)
        .unwrap_or_else(|err| {
            return Err(err).unwrap();
        });

    loader
        .register_template_file(core_mod_name, core_mod_path)
        .unwrap_or_else(|err| {
            return Err(err).unwrap();
        });

    let data = serde_json::json!({
     "name": name,
    });

    create_dir(&core_target_path).unwrap_or_else(|err| {
        return Err(err).unwrap();
    });

    // Now generate the files, shadowing template is fine.
    let template = loader.render(core_name, &data).unwrap_or_else(|err| {
        return Err(err).unwrap();
    });

    write(format!("{}/{}.rs", core_target_path, name), &template)
        .unwrap_or_else(|err| return Err(err).unwrap());

    let template = loader.render(core_mod_name, &data).unwrap_or_else(|err| {
        return Err(err).unwrap();
    });

    write(format!("{}/mod.rs", core_target_path), &template)
        .unwrap_or_else(|err| return Err(err).unwrap());

    Ok(())
}
