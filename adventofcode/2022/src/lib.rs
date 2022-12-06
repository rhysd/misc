#![cfg(test)]

use std::fs;
use std::process::Command;

pub fn run_test(day: u8, part: u8, file: &str, expected: &str) {
    let file = fs::File::open(file).unwrap();
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(day.to_string())
        .arg(part.to_string())
        .stdin(file)
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(stdout.trim(), expected, "stderr: {:?}", stderr);
}

#[cfg(test)]
mod tests {
    macro_rules! test {
        ($day:expr, $part:expr, $expected:expr) => {
            ::paste::paste! {
                #[test]
                fn [<day_ $day _ $part>]() {
                    super::run_test($day, $part, concat!($day, "_test.txt"), $expected);
                }
            }
        };
    }

    test!(1, 1, "24000");
    test!(1, 2, "45000");
    test!(2, 1, "15");
    test!(2, 2, "12");
    test!(3, 1, "157");
    test!(3, 2, "70");
    test!(4, 1, "2");
    test!(4, 2, "4");
    test!(5, 1, "CMZ");
    test!(5, 2, "MCD");
}
