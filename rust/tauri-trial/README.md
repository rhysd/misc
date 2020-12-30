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
