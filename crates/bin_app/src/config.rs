use crate::CONFIG_DIR;
use serde::{Deserialize, Serialize};
use std::cell::LazyCell;
use std::path::PathBuf;

#[warn(dead_code)]
const CONFIG_FILE: LazyCell<PathBuf> =
    LazyCell::new(|| PathBuf::from(CONFIG_DIR.join("config.toml")));

#[derive(Serialize, Deserialize)]
pub struct Config
{

}