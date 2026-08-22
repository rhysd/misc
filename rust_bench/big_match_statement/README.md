This benchmark demonstrates how big `match` statements can be optimized.

## Benchmark result

| Benchmark                   | Time                              | Throughput                                 |
|-----------------------------|-----------------------------------|--------------------------------------------|
| `commands/baseline`         | `[11.832 µs 11.914 µs 11.994 µs]` | `[1.2103 GiB/s 1.2185 GiB/s 1.2269 GiB/s]` |
| `commands/two_phase`        | `[7.1972 µs 7.2239 µs 7.2530 µs]` | `[2.0015 GiB/s 2.0095 GiB/s 2.0170 GiB/s]` |
| `commands/flatten`          | `[6.4869 µs 6.5049 µs 6.5232 µs]` | `[2.2254 GiB/s 2.2316 GiB/s 2.2378 GiB/s]` |
| `rust_coding/baseline`      | `[43.962 µs 44.156 µs 44.387 µs]` | `[1.3565 GiB/s 1.3636 GiB/s 1.3696 GiB/s]` |
| `rust_coding/two_phase`     | `[28.683 µs 28.750 µs 28.821 µs]` | `[2.0891 GiB/s 2.0943 GiB/s 2.0992 GiB/s]` |
| `rust_coding/flatten`       | `[24.789 µs 24.844 µs 24.903 µs]` | `[2.4178 GiB/s 2.4235 GiB/s 2.4289 GiB/s]` |
| `scroll/baseline`           | `[46.488 µs 46.565 µs 46.646 µs]` | `[799.63 MiB/s 801.02 MiB/s 802.34 MiB/s]` |
| `scroll/two_phase`          | `[28.118 µs 28.182 µs 28.250 µs]` | `[1.2894 GiB/s 1.2925 GiB/s 1.2954 GiB/s]` |
| `scroll/flatten`            | `[26.893 µs 26.953 µs 27.015 µs]` | `[1.3483 GiB/s 1.3514 GiB/s 1.3544 GiB/s]` |
| `set_lines/baseline`        | `[4.0375 µs 4.0447 µs 4.0524 µs]` | `[966.75 MiB/s 968.61 MiB/s 970.33 MiB/s]` |
| `set_lines/two_phase`       | `[2.9224 µs 2.9271 µs 2.9315 µs]` | `[1.3051 GiB/s 1.3071 GiB/s 1.3092 GiB/s]` |
| `set_lines/flatten`         | `[2.8665 µs 2.8720 µs 2.8779 µs]` | `[1.3294 GiB/s 1.3321 GiB/s 1.3347 GiB/s]` |
| `ui_init/baseline`          | `[5.4833 µs 5.5024 µs 5.5248 µs]` | `[1.6747 GiB/s 1.6816 GiB/s 1.6874 GiB/s]` |
| `ui_init/two_phase`         | `[3.4191 µs 3.4256 µs 3.4326 µs]` | `[2.6955 GiB/s 2.7010 GiB/s 2.7062 GiB/s]` |
| `ui_init/flatten`           | `[3.2447 µs 3.2542 µs 3.2646 µs]` | `[2.8343 GiB/s 2.8433 GiB/s 2.8516 GiB/s]` |
| `ui_init_400x100/baseline`  | `[126.34 µs 127.17 µs 128.58 µs]` | `[969.17 MiB/s 979.97 MiB/s 986.39 MiB/s]` |
| `ui_init_400x100/two_phase` | `[94.602 µs 94.800 µs 94.993 µs]` | `[1.2811 GiB/s 1.2837 GiB/s 1.2864 GiB/s]` |
| `ui_init_400x100/flatten`   | `[94.740 µs 94.923 µs 95.113 µs]` | `[1.2795 GiB/s 1.2821 GiB/s 1.2846 GiB/s]` |

Environment:

- Chip: Apple M4
- Rust: 1.98.0
