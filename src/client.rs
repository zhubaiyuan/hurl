use crate::app::{App, Method, Parameter};
use crate::errors::{Error, HurlResult};
use log::{info, debug, trace, log_enabled, self};
use reqwest::multipart::Form;
use reqwest::{Client, RequestBuilder, Response, Url};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

pub fn perform_method(
    app: &App,
    method: &Method,
    session: &mut Option<Session>,
) -> HurlResult<Response> {
    let method_data = method.data();
    perform(
        app,
        method.into(),
        session,
        &method_data.url,
        &method_data.parameters,
    )
}
