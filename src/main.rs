use yup_oauth2 as oauth2;

#[derive(Debug, failure::Fail)]
pub enum Error {
    #[fail(display = "Missing GOOGLE_CREDENTIALS")]
    MissingKey,
    #[fail(display = "IO Error: {:#?}", _0)]
    IOError(std::io::Error),
    #[fail(display = "JSON Error: {}", 0)]
    JsonError(serde_json::Error),
    #[fail(display = "OAuth Error: {}", 0)]
    OauthError(yup_oauth2::error::Error),
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

impl From<yup_oauth2::error::Error> for Error {
    fn from(err: yup_oauth2::error::Error) -> Self {
        Error::OauthError(err)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let key_file = std::env::var_os("GOOGLE_CREDENTIALS")
        .ok_or_else(|| Error::MissingKey)?
        .to_string_lossy()
        .into_owned();
    let key = oauth2::read_service_account_key(&key_file).await?;

    let authenticator = oauth2::ServiceAccountAuthenticator::builder(key)
        .build()
        .await?;

    // let https = HttpsConnector::new(1)?;
    // let client = Client::builder().keep_alive(false).build(https);

    let token = authenticator
        .token(&["https://www.googleapis.com/auth/cloud-platform"])
        .await?;

    let json = serde_json::to_string_pretty(&token)?;
    println!("{}", json);
    Ok(())
}
