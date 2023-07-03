#![windows_subsystem = "windows"]
use std::collections::HashMap;
use std::io::Write;
use winapi::um::winuser::{GetForegroundWindow, GetWindowTextW};
use std::time::SystemTime;
use std::fs::{ File};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize,Clone)]
struct Item {
    name: String,
    start_time: u64,
    duration: u64,
}

fn main() {
    let mut info: HashMap<String, Item> = HashMap::new();



    // keep track of current window title

    let mut current_window_title = String::new();
    let up_time = SystemTime::now();
    let mut timer = SystemTime::now();

    // save to Desktop
    let desktop = std::env::var("USERPROFILE").unwrap() + "\\OneDrive/Documents/Traco/\\";

    // check if folder exists
    if !std::path::Path::new(&desktop).exists() {
        std::fs::create_dir(&desktop).unwrap();
    }


    loop {
        // get current window title

        let window_title = get_window_title();

        if info.contains_key(&window_title) {
            let mut item = info.get_mut(&window_title).unwrap();
            item.duration += 1;
        }else{
            let item = Item{
                name: window_title.clone(),
                start_time: up_time.elapsed().unwrap().as_secs() + up_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(), 
                duration: 1,
            };
            info.insert(window_title.clone(), item);

        }


        // if window title has changed
        if window_title != current_window_title {
            // print window title

            println!("{}", window_title);

            // update current window title

            current_window_title = window_title;
        }
    
        // every 5 seconds

        println!("{} seconds left", timer.elapsed().unwrap().as_secs());
        if timer.elapsed().unwrap().as_secs() > 5
        {
            // let mut file = File::create("data.json").unwrap();
            let mut file = File::create(desktop.clone() + "data.json").unwrap();
            let json = serde_json::to_string(&info).unwrap();
            file.write_all(json.as_bytes()).unwrap();
            println!("Saved");
            timer = SystemTime::now();
        }

        // sleep for 1 second

        std::thread::sleep(std::time::Duration::from_secs(1));

    }

    // gracefully handle exit



}

fn get_window_title() -> String {
    unsafe {
        let hwnd = GetForegroundWindow();

        let mut title = [0u16; 512];

        GetWindowTextW(hwnd, title.as_mut_ptr(), 512);

        let title = String::from_utf16_lossy(&title);

        title.to_string().replace("\u{0}", "")
    }
}
