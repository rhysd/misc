use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn bench(c: &mut Criterion) {
    let input = fs::read_to_string("test.md").unwrap();

    c.bench_function("pulldown-cmark", |b| {
        b.iter(|| {
            use pulldown_cmark::*;
            let mut options = Options::empty();
            options.insert(
                Options::ENABLE_STRIKETHROUGH
                    | Options::ENABLE_TASKLISTS
                    | Options::ENABLE_FOOTNOTES,
            );
            let parser = Parser::new(&input);
            let mut output = String::new();
            html::push_html(&mut output, parser);
            assert!(output.len() > 0);
        })
    });

    c.bench_function("comrak", |b| {
        b.iter(|| {
            use comrak::{markdown_to_html, ComrakOptions};
            let mut options = ComrakOptions::default();
            options.extension.strikethrough = true;
            options.extension.table = true;
            options.extension.tasklist = true;
            options.extension.footnotes = true;
            let output = markdown_to_html(&input, &options);
            assert!(output.len() > 0);
        })
    });

    c.bench_function("cmark-gfm", |b| {
        b.iter(|| {
            use cmark_gfm::{Options, Parser, Render};

            let mut options = Options::empty();
            options.insert(Options::CMARK_OPT_FOOTNOTES);

            let parser = Parser::new(options);

            let extensions = &["table", "strikethrough", "tasklist"];
            for extension in extensions {
                parser.add_extension(extension).unwrap();
            }

            parser.parse(&input);
            let result = Render::to_html(&parser);
            assert!(result.len() > 0);
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
