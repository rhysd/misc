Glyph rasterization benchmark
=============================

This is a benchmark suite to measure glyph rasterization among several APIs:

- [swash](https://crates.io/crates/swash) crate
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
                        time:   [3.2771 µs 3.2860 µs 3.2957 µs]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
glyph_rasterize/latin_A/directwrite/12px
                        time:   [682.01 ns 683.79 ns 685.75 ns]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
glyph_rasterize/latin_A/swash/24px
                        time:   [3.6004 µs 3.6152 µs 3.6324 µs]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
glyph_rasterize/latin_A/directwrite/24px
                        time:   [1.7890 µs 1.7942 µs 1.8001 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
glyph_rasterize/latin_A/swash/48px
                        time:   [4.1454 µs 4.1605 µs 4.1768 µs]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
glyph_rasterize/latin_A/directwrite/48px
                        time:   [5.0869 µs 5.0990 µs 5.1120 µs]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe
glyph_rasterize/latin_A/swash/96px
                        time:   [5.2014 µs 5.2222 µs 5.2462 µs]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
glyph_rasterize/latin_A/directwrite/96px
                        time:   [17.782 µs 17.816 µs 17.850 µs]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

glyph_rasterize/cjk_kanji/swash/12px
                        time:   [4.8920 µs 4.9050 µs 4.9182 µs]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high severe
glyph_rasterize/cjk_kanji/directwrite/12px
                        time:   [914.63 ns 917.25 ns 920.32 ns]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
glyph_rasterize/cjk_kanji/swash/24px
                        time:   [5.7077 µs 5.7212 µs 5.7360 µs]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
glyph_rasterize/cjk_kanji/directwrite/24px
                        time:   [2.8354 µs 2.8414 µs 2.8484 µs]
Found 12 outliers among 100 measurements (12.00%)
  8 (8.00%) high mild
  4 (4.00%) high severe
glyph_rasterize/cjk_kanji/swash/48px
                        time:   [7.7450 µs 7.7624 µs 7.7814 µs]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
glyph_rasterize/cjk_kanji/directwrite/48px
                        time:   [8.6815 µs 8.7162 µs 8.7626 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
glyph_rasterize/cjk_kanji/swash/96px
                        time:   [11.750 µs 11.787 µs 11.830 µs]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high severe
glyph_rasterize/cjk_kanji/directwrite/96px
                        time:   [30.670 µs 30.766 µs 30.866 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

glyph_rasterize/latin_fi_ligature/swash/12px
                        time:   [4.5696 µs 4.6311 µs 4.7039 µs]
Found 19 outliers among 100 measurements (19.00%)
  2 (2.00%) high mild
  17 (17.00%) high severe
glyph_rasterize/latin_fi_ligature/directwrite/12px
                        time:   [666.57 ns 668.62 ns 671.02 ns]
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe
glyph_rasterize/latin_fi_ligature/swash/24px
                        time:   [4.8782 µs 4.9378 µs 5.0099 µs]
Found 18 outliers among 100 measurements (18.00%)
  18 (18.00%) high severe
glyph_rasterize/latin_fi_ligature/directwrite/24px
                        time:   [1.3968 µs 1.4023 µs 1.4083 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
glyph_rasterize/latin_fi_ligature/swash/48px
                        time:   [5.4928 µs 5.5537 µs 5.6262 µs]
Found 20 outliers among 100 measurements (20.00%)
  2 (2.00%) high mild
  18 (18.00%) high severe
glyph_rasterize/latin_fi_ligature/directwrite/48px
                        time:   [3.9973 µs 4.0113 µs 4.0312 µs]
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
glyph_rasterize/latin_fi_ligature/swash/96px
                        time:   [6.7080 µs 6.7705 µs 6.8419 µs]
Found 18 outliers among 100 measurements (18.00%)
  13 (13.00%) high mild
  5 (5.00%) high severe
glyph_rasterize/latin_fi_ligature/directwrite/96px
                        time:   [13.564 µs 13.669 µs 13.769 µs]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

glyph_rasterize/emoji_grinning_face/swash/12px
                        time:   [4.2560 µs 4.2653 µs 4.2759 µs]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe
glyph_rasterize/emoji_grinning_face/directwrite/12px
                        time:   [1.2231 µs 1.2270 µs 1.2317 µs]
Found 9 outliers among 100 measurements (9.00%)
  8 (8.00%) high mild
  1 (1.00%) high severe
glyph_rasterize/emoji_grinning_face/swash/24px
                        time:   [6.1565 µs 6.1739 µs 6.1938 µs]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
glyph_rasterize/emoji_grinning_face/directwrite/24px
                        time:   [3.6199 µs 3.6317 µs 3.6435 µs]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
glyph_rasterize/emoji_grinning_face/swash/48px
                        time:   [11.682 µs 11.714 µs 11.748 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
glyph_rasterize/emoji_grinning_face/directwrite/48px
                        time:   [12.042 µs 12.096 µs 12.157 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
glyph_rasterize/emoji_grinning_face/swash/96px
                        time:   [29.519 µs 29.604 µs 29.709 µs]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
glyph_rasterize/emoji_grinning_face/directwrite/96px
                        time:   [44.067 µs 44.156 µs 44.250 µs]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
```
