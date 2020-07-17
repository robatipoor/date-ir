#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::env;

const TIME_IR_URL: &str = "https://www.time.ir";
const DEFAULT_DATE: &str = "SHAMSI";
const FARSI_NUMBERS: [char; 10] = ['۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹'];

const SHAMSI_PATTERN: &str = r#"<span id="ctl00_cphTop_Sampa_Web_View_TimeUI_ShowDate00cphTop_3734_lblShamsiNumeral" class="show numeral">(\d{4}/\d{2}/\d{2})</span>"#;
const HIJRI_PATTERN: &str = r#"<span id="ctl00_cphTop_Sampa_Web_View_TimeUI_ShowDate00cphTop_3734_lblHijriNumeral" class="show numeral">(\d{4}/\d{2}/\d{2})</span>"#;
const GREGORIAN_PATTERN: &str = r#"<span id="ctl00_cphTop_Sampa_Web_View_TimeUI_ShowDate00cphTop_3734_lblGregorianNumeral" class="show numeral">(\d{4}-\d{2}-\d{2})</span>"#;

lazy_static! {
    static ref SHAMSI_REGEX: Regex = Regex::new(SHAMSI_PATTERN).unwrap();
    static ref HIJRI_REGEX: Regex = Regex::new(HIJRI_PATTERN).unwrap();
    static ref GREGORIAN_REGEX: Regex = Regex::new(GREGORIAN_PATTERN).unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(TIME_IR_URL).await?.text().await?;

    let args: Vec<String> = env::args().collect();
    let arg = args
        .get(1)
        .or(Some(&DEFAULT_DATE.to_owned()))
        .map(|s| s.to_ascii_uppercase())
        .unwrap();
    match &*arg {
        "SHAMSI" => {
            for cap in SHAMSI_REGEX.captures_iter(&*resp) {
                println!("{}", convert_farsi_date_to_english_date(cap[1].to_string()));
            }
        }
        "HIJRI" => {
            for cap in HIJRI_REGEX.captures_iter(&*resp) {
                println!("{}", convert_farsi_date_to_english_date(cap[1].to_string()));
            }
        }
        "GREGORIAN" => {
            for cap in GREGORIAN_REGEX.captures_iter(&*resp) {
                println!("{}", &cap[1]);
            }
        }
        _ => panic!("input invalid !"),
    }

    Ok(())
}

fn convert_farsi_date_to_english_date(mut date_input: String) -> String {
    let mut index = 0;
    for c in FARSI_NUMBERS.iter() {
        date_input = date_input.replace(*c, &*index.to_string());
        index += 1;
    }
    date_input
}

#[test]
fn test_convert_farsi_date_to_english_date() {
    assert_eq!(
        "1399/04/27",
        convert_farsi_date_to_english_date("۱۳۹۹/۰۴/۲۷".to_string())
    );
    assert_eq!(
        "1399/04/25",
        convert_farsi_date_to_english_date("۱۳۹۹/۰۴/۲۵".to_string())
    );
    assert_ne!(
        "1399/04/0۲",
        convert_farsi_date_to_english_date("۱۳۹۹/۰۴/۰۲".to_string())
    );
    assert_eq!(
        "1399/04/03",
        convert_farsi_date_to_english_date("۱۳۹۹/۰۴/۰3".to_string())
    );
}
