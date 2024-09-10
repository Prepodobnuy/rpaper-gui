use serde_json::Value;
use std::fs::File;
use std::io::Read;

pub struct Params {
    pub change_colorscheme: bool,
    pub apply_templates: bool,
    pub cache_wallpaper: bool,
    pub set_wallpaper: bool,

    pub change_contrast: bool,
    pub change_brightness: bool,
    pub change_huerotate: bool,
    pub change_blur: bool,
    pub image_flip_h: bool,
    pub image_flip_v: bool,
    pub invert_image: bool,
    
    pub contrast: f64,
    pub brightness: i64,
    pub huerotate: i64,
    pub blur: f64,
}

fn read_data(data_path: &str) -> Value {
    let mut file = File::open(data_path).unwrap();
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).unwrap();

    let data: Value = serde_json::from_str(&json_data).unwrap();

    return data;
}

impl Params {
    pub fn new(config_path: &str) -> Self {
        let config_data: Value = read_data(config_path);

        let change_colorscheme = config_data["cache_colorscheme"].as_bool().unwrap_or(true);
        let apply_templates = config_data["apply_templates"].as_bool().unwrap_or(true);
        let cache_wallpaper = config_data["cache_wallpaper"].as_bool().unwrap_or(true);
        let set_wallpaper = config_data["set_wallpaper"].as_bool().unwrap_or(true);

        let change_contrast = config_data["change_contrast"].as_bool().unwrap_or(false);
        let change_brightness = config_data["change_brightness"].as_bool().unwrap_or(false);
        let change_huerotate = config_data["change_huerotate"].as_bool().unwrap_or(false);
        let change_blur = config_data["change_blur"].as_bool().unwrap_or(false);
        let image_flip_h = config_data["image_flip_h"].as_bool().unwrap_or(false);
        let image_flip_v = config_data["image_flip_v"].as_bool().unwrap_or(false);
        let invert_image = config_data["invert_image"].as_bool().unwrap_or(false);

        let contrast = config_data["contrast"].as_f64().unwrap_or(0.0);
        let brightness = config_data["brightness"].as_i64().unwrap_or(0);
        let huerotate = config_data["huerotate"].as_i64().unwrap_or(0);
        let blur = config_data["blur"].as_f64().unwrap_or(0.0);

        Params{
            change_colorscheme,
            apply_templates,
            cache_wallpaper,
            set_wallpaper,
            change_contrast,
            change_brightness,
            change_huerotate,
            change_blur,
            image_flip_h,
            image_flip_v,
            invert_image,
            contrast,
            brightness,
            huerotate,
            blur,
        }
    }
}