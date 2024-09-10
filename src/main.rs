mod wallpaper;

fn main() {
    let wallpapers = wallpaper::list_images_from_dir("/home/prepodobnuy/Documents/Wallpapers", "");
    for wallpaper in wallpapers {
        println!("{} => {}", wallpaper.tag, wallpaper.path)
    }
}
