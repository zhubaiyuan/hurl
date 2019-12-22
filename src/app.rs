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

impl App {
    pub fn validate(&mut self) -> HurlResult<()> {
        if self.cmd.is_none() && self.url.is_none() {
            return Err(Error::MissingUrlAndCommand);
        }
        Ok(())
    }

    pub fn log_level(&self) -> Option<&'static str> {
        if self.quiet || self.verbose <= 0 {
            return None;
        }

        match self.verbose {
            1 => Some("error"),
            2 => Some("warn"),
            3 => Some("info"),
            4 => Some("debug"),
            _ => Some("trace"),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "screaming_snake_case")]
pub enum Method {
    HEAD(MethodData),
    GET(MethodData),
    PUT(MethodData),
    POST(MethodData),
    PATCH(MethodData),
    DELETE(MethodData),
}

impl Method {
    pub fn data(&self) -> &MethodData {
        use Method::*;
        match self {
            HEAD(x) => x,
            GET(x) => x,
            PUT(x) => x,
            POST(x) => x,
            PATCH(x) => x,
            DELETE(x) => x,
        }
    }
}

#[derive(StructOpt, Debug)]
pub struct MethodData {
    /// The URL to request.
    pub url: String,

    /// The headers, data, and query parameters to add to the request.
    ///
    /// There are seven types of parameters that can be added to a command-line. Each type of
    /// parameter is distinguished by the unique separator between the key and value.
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

#[derive(Debug)]
pub enum Parameter {
    // :
    Header { key: String, value: String },
    // =
    Data { key: String, value: String },
    // :=
    RawJsonData { key: String, value: String },
    // ==
    Query { key: String, value: String },
    // @
    FormFile { key: String, filename: String },
    // =@
    DataFile { key: String, filename: String },
    // :=@
    RawJsonDataFile { key: String, filename: String },
}

#[derive(Debug)]
enum Token<'a> {
    Text(&'a str),
    Escape(char),
}
