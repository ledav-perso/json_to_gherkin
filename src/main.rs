use env_logger::{Builder, Target};
use std::fs;

mod parser;
mod utils;

fn main() {
    println!("Initialization...");

    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    log::debug!("log availables at level {}", log::max_level());

    utils::get_vars();

    const OPENAPI_PATH: &str = "OPENAPI_PATH";
    if let Ok(openapi_path) = utils::get_var(OPENAPI_PATH) {
        let contents =
            fs::read_to_string(openapi_path).expect("Should have been able to read the file");

        parser::parse(&contents).unwrap();
    } else {
        log::error!("no env variable {}", OPENAPI_PATH);
    }
}
