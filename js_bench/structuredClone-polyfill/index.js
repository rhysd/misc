const assert = require('assert');
const Benchmark = require('benchmark');
const structuredClonePolyfill = require('@ungap/structured-clone').default;
const data = require('./test.json');
const lodashCloneDeep = require('lodash.clonedeep');
const rfdc = require('rfdc')({proto: true, circles: false});
const cloneDeep = require('clone-deep');

function structuredCloneJson(x) {
    return JSON.parse(JSON.stringify(x));
}

function structuredCloneHandmade(x) {
    if (typeof x === 'number' || typeof x === 'boolean' || typeof x === 'string' || x === null) {
        return x;
    } else if (Array.isArray(x)) {
        return x.map(structuredCloneHandmade);
    } else {
        const ret = {};
        for (const [k, v] of Object.entries(x)) {
            ret[k] = structuredCloneHandmade(v);
        }
        return ret;
    }
}

function prefllight() {
    {
        const cloned = structuredCloneJson(data);
        assert.notStrictEqual(cloned, data);
        assert.deepStrictEqual(cloned, data);
    }
    {
        const cloned = structuredCloneHandmade(data);
        assert.notStrictEqual(cloned, data);
        assert.deepStrictEqual(cloned, data);
    }
}
prefllight();

const suite = new Benchmark.Suite();
suite.add('Native', function() {
    structuredClone(data);
}).add('Polyfill', function() {
    structuredClonePolyfill(data);
}).add('JSON', function() {
    structuredCloneJson(data);
}).add('Handmade', function() {
    structuredCloneHandmade(data);
}).add('Lodash', function() {
    lodashCloneDeep(data);
}).add('RFDC', function() {
    rfdc(data);
}).add('CloneDeep', function() {
    cloneDeep(data);
}).on('cycle', function(event) {
  console.log(String(event.target));
}).on('complete', function() {
  console.log('Fastest is ' + this.filter('fastest').map('name'));
}).run();
