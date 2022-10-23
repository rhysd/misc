use anyhow::Result;
use getopts::Options;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use wry::application::event::{Event, StartCause, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::WebViewBuilder;

#[derive(Serialize)]
#[serde(tag = "kind")]
#[serde(rename_all = "snake_case")]
enum MessageToWebview<'a> {
    Content { content: &'a str },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "kind")]
#[serde(rename_all = "snake_case")]
enum MessageFromWebview {
    Init,
}

fn usage(options: Options) {
    let program = env::args().next().unwrap();
    let header = format!("Usage: {} [option] FILE", program);
    println!("{}", options.usage(&header));
}

fn file_url(path: PathBuf) -> String {
    #[cfg(not(target_os = "windows"))]
    {
        format!("file://{}", path.display())
    }
    #[cfg(target_os = "windows")]
    {
        let slash = path.to_string_lossy().replace('\\', "/");
        format!("file://{}", slash)
    }
}

fn main() -> Result<()> {
    let mut options = Options::new();
    options.optflag("h", "help", "print this help");
    let matches = options.parse(env::args().skip(1))?;
    if matches.opt_present("h") {
        usage(options);
        return Ok(());
    }

    println!("{}", file_url(env::current_dir()?));

    let arg = matches.free.first().map(fs::read_to_string).transpose()?;
    let debug = env::var("DEBUG").is_ok();
    let url = {
        let mut path = env::current_dir()?;
        path.push("dist");
        path.push("index.html");
        file_url(path)
    };

    let event_loop = EventLoop::with_user_event();
    let window = WindowBuilder::new()
        .with_title("Markdown Preview")
        .build(&event_loop)?;
    let proxy = event_loop.create_proxy();
    let webview = WebViewBuilder::new(window)?
        .with_url(&url)?
        .with_devtools(debug)
        .with_ipc_handler(move |_w, s| {
            let m: MessageFromWebview = serde_json::from_str(&s).unwrap();
            proxy.send_event(m).unwrap();
        })
        .build()?;

    #[cfg(debug_assertions)]
    if debug {
        webview.open_devtools(); // This method is defined in debug build only
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::UserEvent(msg) => match msg {
                MessageFromWebview::Init => {
                    if let Some(content) = &arg {
                        let msg = MessageToWebview::Content { content };
                        let mut buf = b"window.myMarkdownPreview.receive(".to_vec();
                        serde_json::to_writer(&mut buf, &msg).unwrap();
                        buf.push(b')');
                        webview
                            .evaluate_script(&String::from_utf8(buf).unwrap())
                            .unwrap();
                    }
                }
            },
            _ => (),
        }
    });
}