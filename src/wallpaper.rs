use std::fs;
use std::path::Path;

pub struct Wallpaper {
    pub path: String,
    pub tag: String,
}

fn get_tag(directory: &str) -> String {
    directory.split("/").last().unwrap().to_string()
}

pub fn list_images_from_dir(directory: &str, tag: &str) -> Vec<Wallpaper> {
    let mut res: Vec<Wallpaper> = Vec::new();
    let entries = fs::read_dir(directory).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            let path_str = path.to_string_lossy().to_string();
            res.extend(list_images_from_dir(
                &path_str, 
                &get_tag(&path_str),
            ));
            continue;
        }

        if path.is_file() {
            res.push(Wallpaper{
                path: path.to_string_lossy().to_string(),
                tag: String::from(tag),
            })
        }
    }
    res
}