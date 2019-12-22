use structopt::StructOpt;
use heck::TitleCase;
use log::trace;

mod app;
mod client;
mod errors;

type OrderedJson = std::collections::BTreeMap<String, serde_json::Value>;

fn main() -> HurlResult<()> {
    let mut app = app::App::from_args();
    app.validate()?;

    if let Some(level) = app.log_level() {
        std::env::set_var("RUST_LOG", format!("hurl={}", level));
        pretty_env_logger::init();
    }

    match app.cmd {
        Some(ref method) => {
            let resp = client::perform_method(&app, method, &mut session)?;
            handle_response(&app, &ss, theme, resp, &mut session)
        }
        None => {
            let url = app.url.take().unwrap();
            let has_data = app.parameters.iter().any(|p| p.is_data());
            let method = if has_data {
                reqwest::Method::POST
            } else {
                reqwest::Method::GET
            };
            let resp = client::perform(&app, method, &mut session, &url, &app.parameters)?;
            handle_response(&app, &ss, theme, resp, &mut session)
        }
    }
}
