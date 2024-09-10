mod wallpaper;
mod config;
mod utils;

fn main() {
    let config_path: String = utils::parse_path("~/.config/rpaper/config.json");
    let wallpapers_path = utils::get_wallpapers_path();
    let wallpapers = wallpaper::list_images_from_dir(&wallpapers_path, "");
    for wallpaper in wallpapers {
        println!("{} => {}", wallpaper.tag, wallpaper.path)
    }
}
