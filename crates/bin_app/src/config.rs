use std::cell::LazyCell;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::CONFIG_DIR;

const CONFIG_FILE: LazyCell<PathBuf> =
    LazyCell::new(|| PathBuf::from(CONFIG_DIR.join("config.toml")));

#[derive(Serialize, Deserialize)]
pub struct Config
{

}