use handlebars::handlebars_helper;
use rust_starter_pack::foundation::logger::logger::Logger;
use std::fs::{create_dir_all, write};
use std::{env, fmt::Error, path::PathBuf};

// TODO - Potential stores I want this to support out the box. DB (Postgres), Cloud Storage.
// TODO - Also need to clean this function up.

const BASE_CORE_PATH: &str = "/src/business/core/";

handlebars_helper!(upper: |str: String| str[0..1].to_uppercase() + &str[1..]);

pub fn create_store(log: &Logger, command: &str, name: &str) -> Result<(), Error> {
    let message = format!("processing {} with name {}_db", command, name);
    log.info_w(&message, Some(()));

    // Create a handlebars registry to use templates.
    let mut loader = handlebars::Handlebars::new();

    loader.register_helper("upper", Box::new(upper));

    // Define the absolute path.
    let abs_path = PathBuf::from(env::current_dir().unwrap());
    let abs_path = abs_path.to_str().unwrap();

    // Store template and target paths
    let store_template_path = format!(
        "{}/src/app/tools/lumber/templates/store/db_base.hbs",
        abs_path
    );

    let store_mod_path = format!(
        "{}/src/app/tools/lumber/templates/mods/store_mod_base.hbs",
        abs_path
    );

    let store_target_path = format!("{}{}{}/stores/{}_db", abs_path, BASE_CORE_PATH, name, name);

    loader
        .register_template_file("db_base", store_template_path)
        .unwrap_or_else(|err| {
            return Err(err).unwrap();
        });

    loader
        .register_template_file("store_mod_base", store_mod_path)
        .unwrap_or_else(|err| {
            return Err(err).unwrap();
        });

    let data = serde_json::json!({
     "name": name,
    });

    create_dir_all(&store_target_path).unwrap_or_else(|err| {
        return Err(err).unwrap();
    });

    let template = loader.render("db_base", &data).unwrap_or_else(|err| {
        return Err(err).unwrap();
    });

    write(format!("{}/{}_db.rs", store_target_path, name), &template)
        .unwrap_or_else(|err| return Err(err).unwrap());

    let template = loader
        .render("store_mod_base", &data)
        .unwrap_or_else(|err| {
            return Err(err).unwrap();
        });

    write(format!("{}/mod.rs", store_target_path), &template)
        .unwrap_or_else(|err| return Err(err).unwrap());

    Ok(())
}
