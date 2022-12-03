const Benchmark = require('benchmark');
const { createElement } = require('react');

const TITLE = 'this is title';
const STYLE = {
    'display': 'flex',
    'width': '100px',
};
const CONTENT = ['hello, ', 'world'];

new Benchmark.Suite()
    .add('string', function () {
        let s = '<div ';
        s += `title="${TITLE}" `;
        let style = '';
        for (const n of Object.keys(STYLE)) {
            style += `${n}:${STYLE[n]};`;
        }
        s += `style="${style}>`;
        for (const c of CONTENT) {
            s += c;
        }
        s += '</div>';
    })
    .add('react', function () {
        const props = {
            title: TITLE,
            style: STYLE,
        };
        createElement('div', props, ...CONTENT);
    })
    .on('cycle', function (event) {
        console.log(String(event.target));
    })
    .on('complete', function () {
        console.log(`Fastest is ${this.filter('fastest').map('name')}`);
    })
    .run();
