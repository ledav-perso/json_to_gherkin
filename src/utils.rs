use std::env;

pub fn get_var(key: &str) -> Result<String, ()> {
    match env::var(key) {
        Ok(value) => Ok(value),
        Err(err) => {
            println!("no environment variable {} : {}", key, err);
            Err(())
        }
    }
}

pub fn get_vars() {
    env::vars().for_each(|v| log::info!("{}: {}", v.0, v.1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_var() {
        assert_eq!(get_var("PATH").unwrap(), env::var("PATH").unwrap());
        assert_eq!(get_var("sdfdsfPATH"), Err(()));
    }
}
