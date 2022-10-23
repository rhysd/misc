use anyhow::Result;
use getopts::Options;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use wry::application::event::{Event, StartCause, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::{FileDropEvent, WebView, WebViewBuilder};

#[derive(Serialize)]
#[serde(tag = "kind")]
#[serde(rename_all = "snake_case")]
enum MessageToWebView<'a> {
    Content { content: &'a str },
}

impl<'a> MessageToWebView<'a> {
    fn send_to(&self, webview: &WebView) -> Result<()> {
        let mut buf = b"window.myMarkdownPreview.receive(".to_vec();
        serde_json::to_writer(&mut buf, self)?;
        buf.push(b')');
        webview.evaluate_script(&String::from_utf8(buf).unwrap())?; // XXX: This UTF-8 validation is redundant
        Ok(())
    }

    fn preview(path: impl AsRef<Path>, webview: &WebView) -> Result<()> {
        let content = fs::read_to_string(path.as_ref())?;
        let msg = MessageToWebView::Content { content: &content };
        msg.send_to(webview)
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "kind")]
#[serde(rename_all = "snake_case")]
enum MessageFromWebView {
    Init,
    Open { link: String },
}

#[derive(Debug)]
enum UserEvent {
    FromWebView(MessageFromWebView),
    FileDrop(PathBuf),
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
    let ipc_proxy = event_loop.create_proxy();
    let file_drop_proxy = event_loop.create_proxy();
    let webview = WebViewBuilder::new(window)?
        .with_url(&url)?
        .with_devtools(debug)
        .with_ipc_handler(move |_w, s| {
            let m: MessageFromWebView = serde_json::from_str(&s).unwrap();
            ipc_proxy.send_event(UserEvent::FromWebView(m)).unwrap();
        })
        .with_file_drop_handler(move |_w, e| {
            if let FileDropEvent::Dropped(paths) = e {
                if let Some(path) = paths.into_iter().next() {
                    file_drop_proxy
                        .send_event(UserEvent::FileDrop(path))
                        .unwrap();
                }
            }
            true
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
            Event::UserEvent(event) => match event {
                UserEvent::FromWebView(msg) => match msg {
                    MessageFromWebView::Init => {
                        if let Some(path) = matches.free.first() {
                            MessageToWebView::preview(path, &webview).unwrap();
                        }
                    }
                    MessageFromWebView::Open { link }
                        if link.starts_with("https://") || link.starts_with("http://") =>
                    {
                        open::that(link).unwrap();
                    }
                    MessageFromWebView::Open { mut link } => {
                        if link.starts_with("file://") {
                            link.drain(.."file://".len());
                            #[cfg(target_os = "windows")]
                            {
                                link = link.replace('/', "\\");
                            }
                        }
                        // TODO: Open markdown document in this app
                        let _ = open::that(link);
                    }
                },
                UserEvent::FileDrop(path) => {
                    MessageToWebView::preview(&path, &webview).unwrap();
                }
            },
            _ => (),
        }
    });
}
