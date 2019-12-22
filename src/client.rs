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

pub fn perform(
    app: &App,
    method: reqwest::Method,
    session: &mut Option<Session>,
    raw_url: &str,
    parameters: &Vec<Parameter>,
) -> HurlResult<Response> {
    let client = Client::new();
    let url = parse(app, raw_url)?;
    debug!("Parsed url: {}", url);

    let is_multipart = parameters.iter().any(|p| p.is_form_file());
    if is_multipart {
        trace!("Making multipart request because form file was given");
        if !app.form {
            return Err(Error::NotFormButHasFormFile);
        }
    }

    let mut builder = client.request(method, url);
    builder = handle_session(
        builder,
        session,
        parameters,
        !app.read_only,
        &app.auth,
        &app.token,
    );
    builder = handle_parameters(builder, app.form, is_multipart, parameters)?;
    builder = handle_auth(builder, &app.auth, &app.token)?;

    if log_enabled!(log::Level::Info) {
        let start = Instant::now();
        let result = builder.send().map_err(From::from);
        let elapsed = start.elapsed();
        info!("Elapsed time: {:?}", elapsed);
        result
    } else {
        builder.send().map_err(From::from)
    }
}
