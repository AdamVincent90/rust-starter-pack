use handlebars::handlebars_helper;
use rust_starter_pack::dependency::logger::logger::Logger;
use std::error::Error;
use std::fs::{create_dir_all, write};
use std::{env, path::PathBuf};

// TODO - Potential clients to support out the box. Http (rest) and grpc.
// TODO - Also need to clean this function up.

handlebars_helper!(upper: |str: String| str[0..1].to_uppercase() + &str[1..]);

pub fn create_client(log: &Logger, command: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let message = format!("processing {} with name {}_client", command, name);
    log.info_w(&message, Some("Lumber Create Client"));

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

    // Store template and target paths
    let client_template_path = format!(
        "{}/src/application/tools/lumber/templates/client/client_base.hbs",
        abs_path
    );

    let client_mod_path = format!(
        "{}/src/application/tools/lumber/templates/mods/client_mod_base.hbs",
        abs_path
    );

    let ammended_base_path = format!("/src/business/core/{}/clients/", name);
    let store_target_path = format!("{}{}{}_client", abs_path, ammended_base_path, name);

    match loader.register_template_file("client_base", client_template_path) {
        Ok(loader) => loader,
        Err(err) => {
            return Err(Box::new(err));
        }
    }

    match loader.register_template_file("client_mod_base", client_mod_path) {
        Ok(loader) => loader,
        Err(err) => {
            return Err(Box::new(err));
        }
    }

    let data = serde_json::json!({
     "name": name,
    });

    if let Err(err) = create_dir_all(&store_target_path) {
        return Err(Box::new(err));
    }

    let template = match loader.render("client_base", &data) {
        Ok(template) => template,
        Err(err) => return Err(Box::new(err)),
    };

    if let Err(err) = write(
        format!("{}/{}_client.rs", store_target_path, name),
        &template,
    ) {
        return Err(Box::new(err));
    }

    let template = match loader.render("client_mod_base", &data) {
        Ok(template) => template,
        Err(err) => return Err(Box::new(err)),
    };

    if let Err(err) = write(format!("{}/mod.rs", store_target_path), &template) {
        return Err(Box::new(err));
    }

    Ok(())
}
