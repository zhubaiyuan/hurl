use structopt::StructOpt;
use heck::TitleCase;
use log::trace;

mod app;
mod client;
mod errors;

type OrderedJson = std::collections::BTreeMap<String, serde_json::Value>;
