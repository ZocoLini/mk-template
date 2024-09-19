use std::cell::LazyCell;
use std::env;
use std::path::PathBuf;

mod commands;
mod config;
mod templates;

const BIN_NAME: &str = {
    #[cfg(debug_assertions)]
    {
        "mkt-dev"
    }

    #[cfg(not(debug_assertions))]
    {
        "mkt"
    }
};

const CONFIG_DIR: LazyCell<PathBuf> = LazyCell::new(|| {
    let home = env::var("MKT_HOME");

    let path = if let Ok(hom) = home {
        hom
    } else {
        if cfg!(target_os = "windows") {
            env::var("USERPROFILE").unwrap_or_else(|_| String::from("Home directory not found"))
                + "/."
                + BIN_NAME
        } else {
            env::var("HOME").unwrap_or_else(|_| String::from("Home directory not found"))
                + "/."
                + BIN_NAME
        }
    };

    let path = PathBuf::from(path);

    if !path.exists() {
        std::fs::create_dir(&path).expect("Should create the config dir.");
    }

    path
});

fn main() {
    let command = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let command = command.as_str();

    match commands::try_execute(command) {
        Ok(_) => (),
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}
