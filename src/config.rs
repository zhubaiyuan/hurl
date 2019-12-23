use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

use crate::app::App;
use crate::directories::DIRECTORIES;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub verbose: Option<u8>,
    pub form: Option<bool>,
    pub auth: Option<String>,
    pub token: Option<String>,
    pub secure: Option<bool>,
}
