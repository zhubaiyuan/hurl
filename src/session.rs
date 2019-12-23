use crate::app::{App, Parameter};
use crate::directories::DIRECTORIES;
use crate::errors::HurlResult;
use reqwest::header::COOKIE;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Session {
    path: PathBuf,
    name: String,
    host: String,
    auth: Option<String>,
    token: Option<String>,
    headers: HashMap<String, String>,
    cookies: Vec<(String, String)>,
}
