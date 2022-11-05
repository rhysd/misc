Benchmark for [`structuredClone`](https://developer.mozilla.org/en-US/docs/Web/API/structuredClone) in the following limited conditions:

- No transferables are necessary hence the second argument of `structuredClone` is not used
- Clone only JSON-compatible values

Target is a `package-lock.json` at [eslint/eslint@c3ce5212f672d95dde3465d7d3c4bf99ff665f8b](https://github.com/eslint/eslint/tree/c3ce5212f672d95dde3465d7d3c4bf99ff665f8b) (31726 lines).

Result on Node.js v18.8.0 on macOS 11:

```
Native    x 154 ops/sec ±0.85% (79 runs sampled)
Polyfill  x 154 ops/sec ±0.64% (80 runs sampled)
JSON      x 143 ops/sec ±0.27% (82 runs sampled)
Handmade  x 394 ops/sec ±1.01% (90 runs sampled)
Lodash    x 159 ops/sec ±2.51% (80 runs sampled)
CloneDeep x 244 ops/sec ±0.41% (89 runs sampled)
RFDC      x 624 ops/sec ±0.77% (92 runs sampled)
MyImpl    x 666 ops/sec ±0.36% (94 runs sampled)

Fastest is MyImpl
```
