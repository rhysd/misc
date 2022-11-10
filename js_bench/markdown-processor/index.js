import {unified} from 'unified';
import remarkParse from 'remark-parse';
import remarkFrontmatter from 'remark-frontmatter';
import remarkGfm from 'remark-gfm';
import remarkGemoji from 'remark-gemoji';
import remarkRehype from 'remark-rehype';
import rehypeHighlight from 'rehype-highlight';
import rehypeRaw from 'rehype-raw';
import rehypeStringify from 'rehype-stringify';
import markdownIt from 'markdown-it';
import markdownItEmoji from 'markdown-it-emoji/light.js';
import markdownItTaskLists from 'markdown-it-task-lists';
import markdownItFrontMatter from 'markdown-it-front-matter';
import hljs from 'highlight.js';
import Benchmark from 'benchmark';
import * as fs from 'fs';
import { ok } from 'assert';

const data = fs.readFileSync('test.md', 'utf-8');

const remarkCompiler = unified()
        .use(remarkParse)
        .use(remarkFrontmatter)
        .use(remarkGfm)
        .use(remarkGemoji)
        .use(remarkRehype, { allowDangerousHtml: true })
        .use(rehypeRaw)
        .use(rehypeHighlight, { plainText: ['txt', 'text'] })
        .use(rehypeStringify);

async function remark(src) {
    const file = await remarkCompiler.process(src);
    return String(file);
}

const mdIt = markdownIt({
        html: true,
        langPrefix: 'language-',
        linkify: true,
        highlight(str, lang) {
            if (lang && hljs.getLanguage(lang)) {
                try {
                    return hljs.highlight(str, { language: lang }).value;
                } catch (_) {}
            }

            return ''; // use external default escaping
        }
    })
    .use(markdownItEmoji)
    .use(markdownItTaskLists)
    .use(markdownItFrontMatter);

const suite = new Benchmark.Suite();
suite.add('remark', {
    defer: true,
    fn(deferred) {
        remark(data).then(file => {
            ok(String(file).length > 0);
            deferred.resolve();
        });
    }
}).add('markdown-it', function() {
    const html = mdIt.render(data);
    ok(html.length > 0);
}).on('cycle', function (event) {
    console.log(String(event.target));
}).on('complete', function () {
    console.log('Fastest is ' + this.filter('fastest').map('name'));
}).run();
