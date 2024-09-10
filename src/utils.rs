use std::env;
use std::path::PathBuf;

fn add_home_path_to_string(path: &str) -> String {
    let home_dir = match env::var_os("HOME") {
        Some(dir) => PathBuf::from(dir),
        _none => {
            eprintln!("Error: HOME environment variable is not set.");
            std::process::exit(1);
        }
    };

    return home_dir.join(path).into_os_string().into_string().unwrap();
}

pub fn parse_path(path: &str) -> String {
    if &path[0..1] == "~" {
        return add_home_path_to_string(&path[2..]);
    }
    return String::from(path);
}

pub fn get_wallpapers_path() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let last_arg = &args[args.len() - 1];
        return parse_path(last_arg);
    }
    panic!()
}