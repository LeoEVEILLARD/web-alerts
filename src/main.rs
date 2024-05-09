
use curl::easy::Easy;
use chrono::{DateTime, Local, Utc};
use chrono::offset::MappedLocalTime;
use chrono::prelude::*;
mod utils;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::env;


static SEND_FAILING_MAIL_DAYS_FREQUENCY: i64 = 10;

use regex::Regex;



fn main()
{

    utils::test_mail_config();

    let mut something_new = false;
    let mut succes_email_body =  String::from("Hello, it may be something new in the daily check:\n\n");

    let (sidaris_result, sidaris_message) = check_sidaris();
    something_new = something_new || sidaris_result;
    if sidaris_result 
    {
        succes_email_body.push_str("* ");
        succes_email_body.push_str(&sidaris_message);
    };
  

    //TODO: factorise checkers here next time

    let (whiskey_result, whiskey_message) = check_whiskey_app();
    something_new = something_new || whiskey_result;
    if whiskey_result 
    {
        succes_email_body.push_str("* ");
        succes_email_body.push_str(&whiskey_message);
    } 

    if something_new
    {
        utils::send_mail("Something new!".to_string(), succes_email_body.to_string());
    }

    //send failing mail sometime
    let duration = Utc::now().signed_duration_since(Utc.with_ymd_and_hms(1970, 1, 1, 10, 9, 8).unwrap());
    let days_since_epoch = duration.num_days();
    if days_since_epoch%SEND_FAILING_MAIL_DAYS_FREQUENCY == 0
    {
        utils::send_mail("Nothing new!".to_string(), "Nothing new, but I'm just saying this script is alive, I'll send that mail every {SEND_FAILING_MAIL_DAYS_FREQUENCY} days".to_string());
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

    (!web_content.contains(patern_to_watch), "the holy book seems to be available, please check".to_owned() + sidaris_adress + "\n")
}

fn check_whiskey_app() -> (bool, String)
{
    let whiskey_adress = "https://docs.getwhisky.app/game-support/index.html";


    let exe_path = env::current_exe().ok().unwrap();
    let supported_games_file = exe_path.parent().unwrap().parent().unwrap().parent().unwrap().join("Resources").join("whiskeyapp_supported_games_list.txt");
    let file = File::open(&supported_games_file).expect("cannot open file !");
    let actuals_supported_games: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut easy = Easy::new();
    easy.url(whiskey_adress).unwrap();

    let mut web_content = String::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            web_content.push_str(std::str::from_utf8(data).unwrap());
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    //println!("{}" , web_content);

    let matching_lines: Vec<&str> = web_content.lines().filter(|line| line.starts_with("<li><a href=")).collect();// input_str.lines().filter(|line| line.starts_with("toto")).collect();
    //println!("matching lines {:?}", matching_lines);
    let regex = Regex::new(r"<a[^>]*>([^<]+)</a>").unwrap();

    let mut mail_message = String::from("Hello, It seems to be new games ported on WhiskeyApp : \n");
    let mut new_game_found = false;
    for line in matching_lines.iter() {
            if let Some(captures) = regex.captures(line) {
                // Récupère le texte capturé du premier groupe
                if let Some(text) = captures.get(1) {
                    let text_as_string = &text.as_str().to_string();
                 //   println!("{}", text.as_str());
                    if actuals_supported_games.contains(&text.as_str().to_string()) {
                       // println!("Le nom {} est dans la liste.", &text.as_str().to_string());
                    } else {
                        new_game_found = true;
                        mail_message.push_str("\t* \"");
                        mail_message.push_str(text_as_string);
                        mail_message.push_str("\"\n")
                        //println!("Le nom {} n'est pas dans la liste.", &text.as_str().to_string());
                    }
                }
            }
        }
    mail_message.push_str("please check out ");
    mail_message.push_str(&whiskey_adress);
    mail_message.push_str(" and update reference file");

    //println!("{}", mail_message);
    (new_game_found, mail_message)
}

 
