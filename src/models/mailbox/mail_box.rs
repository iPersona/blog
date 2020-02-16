use lettre::{ClientSecurity, SmtpClient, Transport};

use crate::models::token::Token;
use crate::models::user::Users;
use crate::util::env::Env;
use crate::util::errors::{Error, ErrorCode};
use crate::util::result::InternalStdResult;
use diesel::PgConnection;
use lettre::builder::EmailBuilder;
use log::{debug, error};

pub struct EmailNotify {
    pub email: String,
    pub user_name: String,
}

impl EmailNotify {
    pub fn send_verify_mail(&self, token: &str) -> bool {
        let email = EmailBuilder::new()
            .to((self.email.as_str(), self.user_name.as_str()))
            .from(Env::get().email.as_str())
            .subject("Sign Up Verification")
            .html(Self::html_content(token))
            .build()
            .unwrap();

        // Open a local connection on port 25
        // let mut mailer = SmtpClient::new(("localhost", 10025), ClientSecurity::None).unwrap().transport();
        let mut mailer = SmtpClient::new(
            (Env::get().smtp_server.as_str(), 10025),
            ClientSecurity::None,
        )
        .unwrap()
        .transport();
        // Send the email
        let result = mailer.send(email);
        match result {
            Ok(_) => {
                debug!("Email sent");
                true
            }
            Err(e) => {
                error!("Could not send email: {:?}", e);
                false
            }
        }
    }

    fn html_content(token: &str) -> String {
        let template = include_str!("comment_notify_email.html");
        let url = format!(
            r#"https://{}/#/verify/{}"#,
            Env::get().domain.as_str(),
            token
        );
        // replace
        let template = template.replace("{{verify_link}}", url.as_str());
        debug!("url: {:?}", url);
        debug!("template: {:?}", template.as_str());
        template
    }

    pub fn verify_token(token: &Token, conn: &PgConnection) -> InternalStdResult<()> {
        let user_id = token.user_id()?;
        let user = Users::user(conn, &user_id);
        match user {
            Some(u) => {
                if token.expired() {
                    // Token expired
                    Err(Error {
                        code: ErrorCode::TokenExpired,
                        detail: format!("Token expired!"),
                    })
                } else if u.id == user_id && token.user_name == u.account && token.email == u.email
                {
                    // Token is valid
                    Ok(())
                } else {
                    Err(Error {
                        code: ErrorCode::InvalidToken,
                        detail: format!("Invalid token detected!"),
                    })
                }
            }
            None => Err(Error {
                code: ErrorCode::UserNotExist,
                detail: format!("User not exist"),
            }),
        }
    }
}
