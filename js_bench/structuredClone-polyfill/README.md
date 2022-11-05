Benchmark for [`structuredClone`](https://developer.mozilla.org/en-US/docs/Web/API/structuredClone) in the following conditions:

- No transferables are necessary hence the second argument of `structuredClone` is not used
- Clone only JSON-compatible values

Target is a `package-lock.json` at [eslint/eslint@c3ce5212f672d95dde3465d7d3c4bf99ff665f8b](https://github.com/eslint/eslint/tree/c3ce5212f672d95dde3465d7d3c4bf99ff665f8b) (31726 lines).

Result on Node.js v18.8.0:

```
Native    x 155 ops/sec ±1.13% (79 runs sampled)
Polyfill  x 156 ops/sec ±0.68% (80 runs sampled)
JSON      x 142 ops/sec ±0.74% (81 runs sampled)
Handmade  x 396 ops/sec ±1.05% (91 runs sampled)
Lodash    x 170 ops/sec ±2.32% (81 runs sampled)
RFDC      x 637 ops/sec ±0.46% (95 runs sampled)
CloneDeep x 242 ops/sec ±1.30% (88 runs sampled)

Fastest is RFDC
```
