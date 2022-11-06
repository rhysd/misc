Benchmark for [`structuredClone`](https://developer.mozilla.org/en-US/docs/Web/API/structuredClone) in the following limited conditions:

- No transferables are necessary hence the second argument of `structuredClone` is not used
- Clone only JSON-compatible values

Target is a `package-lock.json` at [eslint/eslint@c3ce5212f672d95dde3465d7d3c4bf99ff665f8b](https://github.com/eslint/eslint/tree/c3ce5212f672d95dde3465d7d3c4bf99ff665f8b) (31726 lines).

Result on Node.js v18.8.0 on macOS 11:

```
Native       x 153 ops/sec ±0.43% (87 runs sampled)
Polyfill     x 152 ops/sec ±0.73% (78 runs sampled)
JSON         x 142 ops/sec ±0.35% (81 runs sampled)
Handmade     x 400 ops/sec ±0.31% (92 runs sampled)
Lodash       x 159 ops/sec ±2.66% (80 runs sampled)
CloneDeep    x 244 ops/sec ±0.50% (89 runs sampled)
RFDC         x 629 ops/sec ±0.66% (94 runs sampled)
FastestClone x 623 ops/sec ±0.67% (92 runs sampled)
MyImpl       x 673 ops/sec ±0.31% (94 runs sampled)

Fastest is MyImpl
```
