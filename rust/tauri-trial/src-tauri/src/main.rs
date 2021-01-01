#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    print!("Args:");
    for arg in env::args() {
        print!(" {}", arg);
    }
    println!();

    tauri::AppBuilder::new()
        .setup(|webview, _| {
            let mut webview = webview.as_mut();
            thread::spawn(move || {
                let mut count = 0;
                loop {
                    thread::sleep(Duration::from_secs(1));
                    tauri::event::emit(&mut webview, "count", Some(format!("{}", count))).unwrap();
                    println!("count: {}", count);
                    count += 1;
                }
            });
        })
        .invoke_handler(|_webview, arg| {
            use cmd::Cmd::*;
            match serde_json::from_str(arg) {
                Err(e) => Err(e.to_string()),
                Ok(command) => {
                    match command {
                        // definitions for your custom commands from Cmd here
                        Greet { message } => {
                            //  your command code
                            println!("message from webview: {}", message);
                        }
                        KeyPress { ctrl, meta, key } => {
                            print!("Key press: ");
                            if ctrl {
                                print!("Ctrl+");
                            }
                            if meta {
                                print!("Meta+");
                            }
                            println!("{}", key);
                        }
                    }
                    Ok(())
                }
            }
        })
        .build()
        .run();
}
