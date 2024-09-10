mod wallpaper;
mod config;
mod utils;
mod localization;

fn main() {
    let config_path: String = utils::parse_path("~/.config/rpaper/config.json");
    let config = config::Params::new(&config_path);

    let wallpapers_path = utils::get_wallpapers_path();
    let wallpapers = wallpaper::list_images_from_dir(&wallpapers_path, "");
}