use log::{debug, error};
use reqwest;

pub struct Recaptcha {
    secret: String,
    response: String,
    remoteip: Option<String>,
}

impl Recaptcha {
    pub fn new(secret: &str, response: &str, remoteip: Option<&str>) -> Self {
        Recaptcha {
            secret: String::from(secret),
            response: String::from(response),
            remoteip: if remoteip.is_some() {
                Some(String::from(remoteip.unwrap()))
            } else {
                None
            },
        }
    }

    pub fn verify(&self) -> bool {
        use std::collections::HashMap;
        let mut params = HashMap::new();
        params.insert("secret", self.secret.clone());
        params.insert("response", self.response.clone());
        match &self.remoteip {
            Some(r) => {
                params.insert("remoteip", r.clone());
            }
            None => {}
        }
        let client = self.proxy_client().unwrap();
        let res = client
            .post("https://www.google.com/recaptcha/api/siteverify")
            .form(&params)
            .send();
        match res {
            Ok(mut r) => {
                let result = r.json::<VerifyResult>();
                match result {
                    Ok(r) => r.is_ok(),
                    Err(e) => {
                        error!("Deserialize reCHAPTCHA result failed: {:?}", e);
                        false
                    }
                }
            }
            Err(e) => {
                error!("Send reCHAPTCHA request failed: {:?}", e);
                false
            }
        }
    }

    fn proxy_client(&self) -> reqwest::Result<reqwest::Client> {
        if cfg!(debug_assertions) {
            let proxy = reqwest::Proxy::https("http://127.0.0.1:1082");
            match proxy {
                Ok(p) => reqwest::Client::builder().proxy(p).build(),
                Err(e) => {
                    error!("generate proxy failed: {:?}", e);
                    Err(e)
                }
            }
        } else {
            reqwest::Client::builder().build()
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerifyResult {
    success: bool,
    challenge_ts: String, // timestamp of the challenge load (ISO format yyyy-MM-dd'T'HH:mm:ssZZ)
    hostname: String,     // the hostname of the site where the reCAPTCHA was solved
    #[serde(rename = "error-codes")]
    error_codes: Option<Vec<String>>, // optional
}

impl VerifyResult {
    pub fn is_ok(&self) -> bool {
        debug!("verify-result: {:?}", self.success);
        self.success
    }
}

pub fn verify_recaptcha(response: &str) -> bool {
    use crate::util::env::Env;
    let secret = Env::get().recaptcha_secret;
    let recaptcha = Recaptcha::new(secret.as_str(), response, None);
    recaptcha.verify()
}
