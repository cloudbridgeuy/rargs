use color_eyre::eyre::Result;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use serde::Serialize;
use tera::{Context, Tera};

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.tera") {
            Ok(t) => t,
            Err(e) => {
                log::error!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        for entry in TEMPLATES_DIR.files() {
            match tera.add_raw_template(
                entry.path().to_str().unwrap(),
                entry.contents_utf8().unwrap(),
            ) {
                Ok(_) => (),
                Err(e) => {
                    log::error!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            }
        }

        tera
    };
}

/// Render the main.sh template
pub fn render<T: Serialize>(context: &T) -> Result<String> {
    let output = TEMPLATES.render("main.tera", &Context::from_serialize(context)?)?;
    Ok(output)
}
