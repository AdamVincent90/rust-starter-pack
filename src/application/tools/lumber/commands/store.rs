use handlebars::handlebars_helper;
use rust_starter_pack::dependency::logger::logger::Logger;
use std::error::Error;
use std::fs::{create_dir_all, write};
use std::{env, path::PathBuf};

// TODO - Potential stores I want this to support out the box. DB (Postgres), Cloud Storage.
// TODO - Also need to clean this function up.

handlebars_helper!(upper: |str: String| str[0..1].to_uppercase() + &str[1..]);

pub fn create_store(log: &Logger, command: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let message = format!("processing {} with name {}_db", command, name);
    log.info_w(&message, Some(()));

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
    ();

    // Store template and target paths
    let store_template_path = format!(
        "{}/src/application/tools/lumber/templates/store/store_base.hbs",
        abs_path
    );

    let store_mod_path = format!(
        "{}/src/application/tools/lumber/templates/mods/store_mod_base.hbs",
        abs_path
    );

    let ammended_base_path = format!("/src/business/core/{}/stores/", name);
    let store_target_path = format!("{}{}{}_db", abs_path, ammended_base_path, name);

    match loader.register_template_file("store_base", store_template_path) {
        Ok(loader) => loader,
        Err(err) => {
            return Err(Box::new(err));
        }
    }

    match loader.register_template_file("store_mod_base", store_mod_path) {
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

    let template = match loader.render("store_base", &data) {
        Ok(loader) => loader,
        Err(err) => {
            return Err(Box::new(err));
        }
    };

    if let Err(err) = write(format!("{}/{}_db.rs", store_target_path, name), &template) {
        return Err(Box::new(err));
    }

    let template = match loader.render("store_mod_base", &data) {
        Ok(loader) => loader,
        Err(err) => {
            return Err(Box::new(err));
        }
    };

    if let Err(err) = write(format!("{}/mod.rs", store_target_path), &template) {
        return Err(Box::new(err));
    }

    Ok(())
}
