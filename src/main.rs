use yup_oauth2 as oauth2;

use failure::Fail;
use futures::future::Future;
use hyper::Client;
use hyper_tls::{self, HttpsConnector};
use oauth2::{GetToken, ServiceAccountAccess};
use serde_json;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Missing GOOGLE_CREDENTIALS")]
    MissingKey,
    #[fail(display = "IO Error: {:#?}", _0)]
    IOError(std::io::Error),
    #[fail(display = "{}", _0)]
    GenericError(String),
    #[fail(display = "JSON Error: {}", 0)]
    JsonError(serde_json::Error),
    #[fail(display = "TLS Error: {}", 0)]
    TlsError(#[cause] hyper_tls::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err)
    }
}

impl From<hyper_tls::Error> for Error {
    fn from(err: hyper_tls::Error) -> Error {
        Error::TlsError(err)
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();

    let key_file = std::env::var_os("GOOGLE_CREDENTIALS")
        .ok_or_else(|| Error::MissingKey)?
        .to_string_lossy()
        .into_owned();
    let key = oauth2::service_account_key_from_file(&key_file)?;

    let https = HttpsConnector::new(1)?;
    let client = Client::builder().keep_alive(false).build(https);

    let mut access = ServiceAccountAccess::new(key, client);
    let token = access
        .token(["https://www.googleapis.com/auth/cloud-platform"].iter())
        .map_err(|e| {
            let e = Error::GenericError(format!("Error getting token: {}", e));
            println!("{}", e);
        })
        .and_then(|token| {
            let json = serde_json::to_string_pretty(&token);
            match json {
                Ok(json) => {
                    println!("{}", json);
                    Err(())
                }
                Err(e) => {
                    println!("{:#?}", e);
                    Ok(())
                }
            }
        });

    tokio::run(token);

    Ok(())
}
