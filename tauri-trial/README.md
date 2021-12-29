Run dev server:

```sh
# Starts web server at localhost:4000
npm run serve

# Start webview app which opens localhost:4000
npm run tauri dev
```

Right-click in the webview opens a web inspector.

Bundle app for distribution:

```sh
# Create debug build
npm run tauri bundle -- --debug

# Create release build
npm run tauri bundle
```

### Memo

There is no way to load local file directly inside Webview. For example:

```html
<img src="file:/Users/rhysd/Develop/github.com/rhysd/misc/rust/tauri-trial/test.jpg"/>
```

Since Webview does not allow to load local resources, this causes an error. Tauri catches the error inside its event
listener and tries to get the resource via `loadAsset` command. The `loadAsset` command expects Rust side to load the
asset file content and send it to Webview side as base64-encoded string. It would make overhead in contrast on loading
it directly.
