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
    let path = PathBuf::from(env::var("HOME").expect("Should have a HOME var.") + "/." + BIN_NAME);
    
    if !path.exists()
    {
        std::fs::create_dir(&path).expect("Should create the config dir.");
    }
    
    path
});

fn main()
{
    let command = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let command = command.as_str();

    match commands::try_execute(command) { 
        Ok(_) => (),
        Err(err) => {
            println!("Error: {:?}", err);
        },
    }
}
