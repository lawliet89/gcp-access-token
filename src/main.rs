use yup_oauth2 as oauth2;

use failure::Fail;
use hyper::net::HttpsConnector;
use hyper_native_tls::{NativeTlsClient};
use native_tls::Error as TlsError;
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
    TlsError(String)
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

impl From<TlsError> for Error {
    fn from(err: TlsError) -> Error {
        Error::TlsError(err.to_string())
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();

    let key_file = std::env::var_os("GOOGLE_CREDENTIALS")
        .ok_or_else(|| Error::MissingKey)?
        .to_string_lossy()
        .into_owned();
    let key = oauth2::service_account_key_from_file(&key_file)?;

    let client =
        hyper::Client::with_connector(HttpsConnector::new(NativeTlsClient::new()?));
    let mut access = ServiceAccountAccess::new(key, client);
    let token = access
        .token(&vec!["https://www.googleapis.com/auth/pubsub"])
        .map_err(|e| Error::GenericError(format!("Error getting token: {}", e)))?;
    println!("{}", serde_json::to_string_pretty(&token)?);
    Ok(())
}
