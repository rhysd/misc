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
                        time:   [3.2942 µs 3.3040 µs 3.3140 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/latin_A/font-kit/12px
                        time:   [468.15 ns 469.08 ns 470.07 ns]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/latin_A/directwrite/12px
                        time:   [685.90 ns 689.68 ns 694.36 ns]
Found 11 outliers among 100 measurements (11.00%)
  7 (7.00%) high mild
  4 (4.00%) high severe
glyph_rasterize/latin_A/swash/24px
                        time:   [3.6437 µs 3.6561 µs 3.6696 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/latin_A/font-kit/24px
                        time:   [798.42 ns 801.14 ns 804.55 ns]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/latin_A/directwrite/24px
                        time:   [1.7970 µs 1.8020 µs 1.8074 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/latin_A/swash/48px
                        time:   [4.1852 µs 4.2014 µs 4.2182 µs]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

glyph_rasterize/latin_A/font-kit/48px
                        time:   [1.9397 µs 1.9449 µs 1.9509 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

glyph_rasterize/latin_A/directwrite/48px
                        time:   [5.0693 µs 5.0807 µs 5.0922 µs]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

glyph_rasterize/latin_A/swash/96px
                        time:   [5.2814 µs 5.3002 µs 5.3209 µs]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/latin_A/font-kit/96px
                        time:   [6.4430 µs 6.4686 µs 6.4954 µs]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

glyph_rasterize/latin_A/directwrite/96px
                        time:   [17.808 µs 17.882 µs 17.965 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

glyph_rasterize/cjk_kanji/swash/12px
                        time:   [4.9228 µs 4.9356 µs 4.9487 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

glyph_rasterize/cjk_kanji/font-kit/12px
                        time:   [559.21 ns 560.82 ns 562.46 ns]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  4 (4.00%) high severe

glyph_rasterize/cjk_kanji/directwrite/12px
                        time:   [910.29 ns 912.10 ns 913.84 ns]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

glyph_rasterize/cjk_kanji/swash/24px
                        time:   [5.7812 µs 5.8059 µs 5.8331 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

glyph_rasterize/cjk_kanji/font-kit/24px
                        time:   [1.1727 µs 1.1756 µs 1.1790 µs]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

glyph_rasterize/cjk_kanji/directwrite/24px
                        time:   [2.8293 µs 2.8369 µs 2.8451 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/cjk_kanji/swash/48px
                        time:   [7.8195 µs 7.8421 µs 7.8674 µs]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

glyph_rasterize/cjk_kanji/font-kit/48px
                        time:   [3.1542 µs 3.1638 µs 3.1738 µs]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

glyph_rasterize/cjk_kanji/directwrite/48px
                        time:   [8.6642 µs 8.6894 µs 8.7171 µs]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

glyph_rasterize/cjk_kanji/swash/96px
                        time:   [11.878 µs 11.932 µs 11.991 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/cjk_kanji/font-kit/96px
                        time:   [11.256 µs 11.295 µs 11.337 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/cjk_kanji/directwrite/96px
                        time:   [30.852 µs 30.965 µs 31.088 µs]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

glyph_rasterize/latin_fi_ligature/swash/12px
                        time:   [4.5779 µs 4.5976 µs 4.6188 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/latin_fi_ligature/font-kit/12px
                        time:   [436.23 ns 437.65 ns 439.19 ns]
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/latin_fi_ligature/directwrite/12px
                        time:   [658.29 ns 665.40 ns 675.10 ns]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

glyph_rasterize/latin_fi_ligature/swash/24px
                        time:   [4.8443 µs 4.8582 µs 4.8729 µs]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

glyph_rasterize/latin_fi_ligature/font-kit/24px
                        time:   [706.49 ns 708.83 ns 711.40 ns]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

glyph_rasterize/latin_fi_ligature/directwrite/24px
                        time:   [1.3843 µs 1.3883 µs 1.3926 µs]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/latin_fi_ligature/swash/48px
                        time:   [5.4268 µs 5.4435 µs 5.4610 µs]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

glyph_rasterize/latin_fi_ligature/font-kit/48px
                        time:   [1.5698 µs 1.5748 µs 1.5802 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/latin_fi_ligature/directwrite/48px
                        time:   [3.9873 µs 3.9965 µs 4.0060 µs]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild

glyph_rasterize/latin_fi_ligature/swash/96px
                        time:   [6.6572 µs 6.6837 µs 6.7131 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/latin_fi_ligature/font-kit/96px
                        time:   [4.6672 µs 4.6907 µs 4.7161 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/latin_fi_ligature/directwrite/96px
                        time:   [13.425 µs 13.509 µs 13.607 µs]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

glyph_rasterize/emoji_grinning_face/swash/12px
                        time:   [4.3275 µs 4.3452 µs 4.3658 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

glyph_rasterize/emoji_grinning_face/font-kit/12px
                        time:   [753.79 ns 755.85 ns 758.02 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/emoji_grinning_face/directwrite/12px
                        time:   [1.2236 µs 1.2283 µs 1.2331 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/emoji_grinning_face/swash/24px
                        time:   [6.1905 µs 6.2127 µs 6.2395 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/emoji_grinning_face/font-kit/24px
                        time:   [1.7686 µs 1.7781 µs 1.7882 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

glyph_rasterize/emoji_grinning_face/directwrite/24px
                        time:   [3.6019 µs 3.6160 µs 3.6300 µs]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

glyph_rasterize/emoji_grinning_face/swash/48px
                        time:   [11.855 µs 11.902 µs 11.950 µs]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/emoji_grinning_face/font-kit/48px
                        time:   [5.8808 µs 5.9034 µs 5.9287 µs]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

glyph_rasterize/emoji_grinning_face/directwrite/48px
                        time:   [12.082 µs 12.132 µs 12.185 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

glyph_rasterize/emoji_grinning_face/swash/96px
                        time:   [30.282 µs 30.412 µs 30.551 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

glyph_rasterize/emoji_grinning_face/font-kit/96px
                        time:   [21.895 µs 22.004 µs 22.110 µs]

glyph_rasterize/emoji_grinning_face/directwrite/96px
                        time:   [44.418 µs 44.581 µs 44.755 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
```
