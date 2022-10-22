TS_SRC := $(wildcard web/*.ts)
RS_SRC := $(wildcard src/*.rs)

node_modules:
	npm install

dist/github-markdown.css: node_modules
	cp node_modules/github-markdown-css/github-markdown.css dist/github-markdown.css

dist/hljs-github.css: node_modules
	cp node_modules/highlight.js/styles/github.css dist/hljs-github.css

dist/index.js: node_modules $(TS_SRC)
	npm run ts
	npm run bundle

webapp: dist/github-markdown.css dist/hljs-github.css dist/index.js

target/debug/rust-webview: $(RS_SRC)
	cargo build

target/release/rust-webview: $(RS_SRC)
	cargo build --release

build: webapp target/debug/rust-webview

release: webapp target/release/rust-webview

.PHONY: webapp build release
