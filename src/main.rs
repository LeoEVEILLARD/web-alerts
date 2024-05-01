
use curl::easy::Easy;
use chrono::{DateTime, Local, Utc};
use chrono::offset::MappedLocalTime;
use chrono::prelude::*;
mod utils;

static SEND_FAILING_MAIL_DAYS_FREQUENCY: i64 = 10;
use utils::*;



fn main()
{
    //testing needed env variables
    let mut test = SMTP_USERNAME.to_owned();
    test = SMTP_KEY.to_owned();
    test = SMTP_ADRESS.to_owned();
    test = FROM_ADRESS.to_owned();
    test = TO_ADRESS.to_owned();


    let mut something_new = false;
    let mut succes_email_body = "Hello, it may be something new in the daily check:\n\n";

    let (sidaris_result, sidaris_message) = check_sidaris();
    something_new = something_new || sidaris_result;
    let binding = succes_email_body.to_owned() + "* " + &sidaris_message;

    //TODO: put other checkers here.

    if something_new
    {
        utils::send_mail("Something new!".to_string(), binding.to_string());
    }

    //send failing mail sometime
    let duration = Utc::now().signed_duration_since(Utc.with_ymd_and_hms(1970, 1, 1, 10, 9, 8).unwrap());
    let days_since_epoch = duration.num_days();
    if days_since_epoch%SEND_FAILING_MAIL_DAYS_FREQUENCY == 0
    {
        utils::send_mail("Nothing new!".to_string(), "Nothing new, but I'm just saying this script is alive".to_string());
    }
}

fn check_sidaris() -> (bool, String)
{
    let sidaris_adress = "https://www.andysidaris.com/product-page/bullets-bombs-and-babes-coffee-table-book";
    let patern_to_watch = "Out of Stock";

    let mut easy = Easy::new();
    easy.url(sidaris_adress).unwrap();

    let mut web_content = String::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            web_content.push_str(std::str::from_utf8(data).unwrap());
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    (!web_content.contains(patern_to_watch), "the holy book seems to be available, please check ".to_owned() + sidaris_adress)
}

 
