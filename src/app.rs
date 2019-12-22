use log::{debug, trace};
use std::convert::TryFrom;
use structopt::StructOpt;

use crate::errors::{Error, HurlResult};

/// A command line HTTP client
#[derive(StructOpt, Debug)]
#[structopt(name = "hurl")]
pub struct App {
    /// Activate quiet mode.
    ///
    /// This overrides any verbose settings.
    #[structopt(short, long)]
    pub quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc.).
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Form mode.
    #[structopt(short, long)]
    pub form: bool,

    /// Basic authentication.
    ///
    /// A string of the form `username:password`. If only
    /// `username` is given then you will be prompted
    /// for a password. If you wish to use no password
    /// then use the form `username:`.
    #[structopt(short, long)]
    pub auth: Option<String>,

    /// Bearer token authentication.
    ///
    /// A token which will be sent as "Bearaer <token>" in
    /// the authorization header.
    #[structopt(short, long)]
    pub token: Option<String>,

    /// Default transport.
    ///
    /// If a URL is given without a transport, i.e example.com/foo
    /// http will be used as the transport by default. If this flag
    /// is set then https will be used instead.
    #[structopt(short, long)]
    pub secure: bool,

    /// The HTTP Method to use, one of: HEAD, GET, POST, PUT, PATCH, DELETE.
    #[structopt(subcommand)]
    pub cmd: Option<Method>,

    /// The URL to issue a request to if a method subcommand is not specified.
    pub url: Option<String>,

    /// The parameters for the request if a method subcommand is not specified.
    ///
    /// There are seven types of parameters that can be added to a command-line.
    /// Each type of parameter is distinguished by the unique separator between
    /// the key and value.
    ///
    /// Header -- key:value
    ///
    ///   e.g. X-API-TOKEN:abc123
    ///
    /// File upload -- key@filename
    ///
    ///   this simulates a file upload via multipart/form-data and requires --form
    ///
    /// Query parameter -- key==value
    ///
    ///   e.g. foo==bar becomes example.com?foo=bar
    ///
    /// Data field -- key=value
    ///
    ///   e.g. foo=bar becomes {"foo":"bar"} for JSON or form encoded
    ///
    /// Data field from file -- key=@filename
    ///
    ///   e.g. foo=@bar.txt becomes {"foo":"the contents of bar.txt"} or form encoded
    ///
    /// Raw JSON data where the value should be parsed to JSON first -- key:=value
    ///
    ///   e.g. foo:=[1,2,3] becomes {"foo":[1,2,3]}
    ///
    /// Raw JSON data from file -- key:=@filename
    ///
    ///   e.g. foo:=@bar.json becomes {"foo":{"bar":"this is from bar.json"}}
    #[structopt(parse(try_from_str = parse_param))]
    pub parameters: Vec<Parameter>,
}
