use serde_json::{Result, Value};

pub fn parse(contents: &str) -> Result<()> {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(contents)?;

    // Access parts of the data by indexing with square brackets.
    log::info!("OPENAPI v{}", v["openapi"]);

    if let Some(paths) = v["paths"].as_object() {
        paths.keys().for_each(|path| log::info!("{}", path));
    }

    Ok(())
}
