use crate::config::SmtpConfig;
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use tracing::error;

pub fn send_email(smtp_config: &SmtpConfig, from: String, body: impl Into<String>) {
    let smtp_relay = &smtp_config.relay;
    let smtp_username = &smtp_config.username;
    let smtp_password = &smtp_config.password;

    let smtp_email_admin: Mailbox = match smtp_config.email_admin.parse() {
        Ok(email) => email,
        Err(err) => {
            error!("Could not send email because parsing environment variable `SMTP_EMAIL_ADMIN` failed: {:?}", err);
            return;
        }
    };

    let from: Mailbox = match from.parse() {
        Ok(sender) => sender,
        Err(err) => {
            error!(
                "Could not send email because parsing from failed: {:?}",
                err
            );
            return;
        }
    };

    let email = match Message::builder()
        .from(from.clone())
        .reply_to(from)
        .to(smtp_email_admin.into())
        .subject("Heavy Metal Releases Enquiry")
        .header(ContentType::TEXT_PLAIN)
        .body(body.into())
    {
        Ok(email) => email,
        Err(err) => {
            error!("Build email failed: {:?}", err);
            return;
        }
    };

    let mailer = match SmtpTransport::relay(&smtp_relay) {
        Ok(transport) => {
            let creds = Credentials::new(smtp_username.into(), smtp_password.into());

            transport.credentials(creds).build()
        }
        Err(err) => {
            error!("Failed to set up relay {smtp_relay}: {:?}", err);
            return;
        }
    };

    if let Err(err) = mailer.send(&email) {
        error!("Send email failed: {:?}", err);
    }
}
