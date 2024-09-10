use crate::utils::parse_path;
use std::fs;

pub struct Localization {

}


pub fn get_localization() -> Localization {
    let Localization_path = parse_path("~/.config/rpaper/rpaper_lang");

    let content: &str = &fs::read_to_string(Localization_path).unwrap_or(String::from("en"));
    
    match content {
        "en" => {

        },
        "ru" => {

        },
        _ => {},
    }
    Localization{}
}

pub fn set_localization(lang: &str) {
    let Localization_path = parse_path("~/.config/rpaper/rpaper_lang");

}