use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let home = env::home_dir();
    let conf_path = Path::join(&home.unwrap(), ".config/rustdown/ui");
    let _ = fs::create_dir_all(&conf_path.clone());
    let ui_path = Path::join(&conf_path, "main.qml");

    let _ = fs::copy("./ui/main.qml", ui_path.clone());
    println!("[MSG] {:?} Has been set", ui_path);
}
