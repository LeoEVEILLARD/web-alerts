
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::SmtpTransport;
use lettre::{Message, Transport};

use std::env;


lazy_static::lazy_static! {
    pub static ref SMTP_USERNAME: String = get_env("SMTP_USERNAME");
    pub static ref SMTP_KEY: String = get_env("SMTP_KEY");
    pub static ref SMTP_ADRESS: String = get_env("SMTP_ADRESS");
    pub static ref FROM_ADRESS: String = get_env("FROM_ADRESS");
    pub static ref TO_ADRESS: String = get_env("TO_ADRESS");
}

fn get_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("Please define {}", name))
}


pub fn send_mail(mail_subject :String, mail_body :String) {  


        let email = Message::builder()
        .from(FROM_ADRESS.parse().unwrap())
        .to(TO_ADRESS.parse().unwrap())
        .subject(mail_subject)
        .body(mail_body)
        .unwrap();

    let creds = Credentials::new(SMTP_USERNAME.to_owned(), SMTP_KEY.to_string());
    let mailer = SmtpTransport::relay(&SMTP_ADRESS)
        .unwrap()
        .credentials(creds)
        .build();
    match mailer.send(&email) {
        Ok(_) => {},
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}