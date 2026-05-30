Glyph rasterization benchmark
=============================

This is a benchmark suite to measure glyph rasterization among several APIs:

- [swash](https://crates.io/crates/swash) crate
- [font-kit](https://crates.io/crates/font-kit) crate, using Core Text on macOS, DirectWrite on Windows, and FreeType on Linux
- DirectWrite (via [windows](https://crates.io/crates/windows) crate) on Windows
- Core Text (via [objc2-core-text](https://crates.io/crates/objc2-core-text) crate) on macOS
- FreeType (via [freetype-rs](https://crates.io/crates/freetype-rs) crate) on Linux

against several types of glyphs:

- Single ASCII character
- CJK kanji character
- Ligature
- Emoji

Result on x86_64 Windows:

```
glyph_rasterize/latin_A/swash/12px
                        time:   [3.3216 µs 3.3343 µs 3.3480 µs]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/latin_A/font-kit/12px
                        time:   [473.83 ns 476.21 ns 479.00 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

glyph_rasterize/latin_A/directwrite/12px
                        time:   [389.39 ns 390.15 ns 390.98 ns]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

glyph_rasterize/latin_A/swash/24px
                        time:   [3.6506 µs 3.6689 µs 3.6916 µs]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

glyph_rasterize/latin_A/font-kit/24px
                        time:   [818.59 ns 825.68 ns 832.64 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/latin_A/directwrite/24px
                        time:   [668.94 ns 674.82 ns 682.51 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

glyph_rasterize/latin_A/swash/48px
                        time:   [4.3091 µs 4.3338 µs 4.3571 µs]

glyph_rasterize/latin_A/font-kit/48px
                        time:   [1.9677 µs 1.9813 µs 1.9961 µs]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

glyph_rasterize/latin_A/directwrite/48px
                        time:   [1.5729 µs 1.5833 µs 1.5950 µs]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/cjk_kanji/swash/12px
                        time:   [4.9117 µs 4.9247 µs 4.9384 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high severe

glyph_rasterize/cjk_kanji/font-kit/12px
                        time:   [562.35 ns 564.10 ns 566.22 ns]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

glyph_rasterize/cjk_kanji/directwrite/12px
                        time:   [463.50 ns 465.00 ns 466.67 ns]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/cjk_kanji/swash/24px
                        time:   [5.8497 µs 5.8814 µs 5.9133 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/cjk_kanji/font-kit/24px
                        time:   [1.1799 µs 1.1849 µs 1.1907 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/cjk_kanji/directwrite/24px
                        time:   [952.21 ns 954.34 ns 956.56 ns]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

glyph_rasterize/cjk_kanji/swash/48px
                        time:   [7.8014 µs 7.8221 µs 7.8459 µs]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/cjk_kanji/font-kit/48px
                        time:   [3.1360 µs 3.1442 µs 3.1527 µs]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

glyph_rasterize/cjk_kanji/directwrite/48px
                        time:   [2.4770 µs 2.4818 µs 2.4875 µs]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/latin_fi_ligature/swash/12px
                        time:   [4.5053 µs 4.5250 µs 4.5528 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

glyph_rasterize/latin_fi_ligature/font-kit/12px
                        time:   [435.01 ns 436.08 ns 437.22 ns]
Found 15 outliers among 100 measurements (15.00%)
  5 (5.00%) high mild
  10 (10.00%) high severe

glyph_rasterize/latin_fi_ligature/directwrite/12px
                        time:   [365.85 ns 367.77 ns 369.64 ns]
Found 11 outliers among 100 measurements (11.00%)
  10 (10.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/latin_fi_ligature/swash/24px
                        time:   [4.8910 µs 4.9039 µs 4.9185 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/latin_fi_ligature/font-kit/24px
                        time:   [722.36 ns 729.16 ns 736.44 ns]
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) high mild
  12 (12.00%) high severe

glyph_rasterize/latin_fi_ligature/directwrite/24px
                        time:   [596.70 ns 599.31 ns 602.00 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

glyph_rasterize/latin_fi_ligature/swash/48px
                        time:   [5.4855 µs 5.7112 µs 5.9921 µs]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) high mild
  7 (7.00%) high severe

glyph_rasterize/latin_fi_ligature/font-kit/48px
                        time:   [1.5821 µs 1.5892 µs 1.5970 µs]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/latin_fi_ligature/directwrite/48px
                        time:   [1.2609 µs 1.2645 µs 1.2688 µs]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/emoji_grinning_face/swash/12px
                        time:   [4.3109 µs 4.3240 µs 4.3389 µs]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

glyph_rasterize/emoji_grinning_face/font-kit/12px
                        time:   [754.50 ns 756.72 ns 759.04 ns]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/emoji_grinning_face/directwrite/12px
                        time:   [536.00 ns 537.23 ns 538.47 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/emoji_grinning_face/swash/24px
                        time:   [6.1890 µs 6.2043 µs 6.2218 µs]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

glyph_rasterize/emoji_grinning_face/font-kit/24px
                        time:   [1.7683 µs 1.7734 µs 1.7795 µs]
Found 13 outliers among 100 measurements (13.00%)
  8 (8.00%) high mild
  5 (5.00%) high severe

glyph_rasterize/emoji_grinning_face/directwrite/24px
                        time:   [1.1113 µs 1.1162 µs 1.1213 µs]
Found 14 outliers among 100 measurements (14.00%)
  6 (6.00%) low mild
  5 (5.00%) high mild
  3 (3.00%) high severe

glyph_rasterize/emoji_grinning_face/swash/48px
                        time:   [11.850 µs 11.882 µs 11.917 µs]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/emoji_grinning_face/font-kit/48px
                        time:   [5.8981 µs 5.9114 µs 5.9260 µs]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/emoji_grinning_face/directwrite/48px
                        time:   [3.2628 µs 3.2697 µs 3.2772 µs]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
```
