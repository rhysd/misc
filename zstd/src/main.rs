use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let file = env::args().nth(1).unwrap();
    eprintln!("File: {file}");
    let bytes = fs::read(file).unwrap();
    eprintln!("Input: {} bytes", bytes.len());

    println!("| Level | Encode (msec) | Encoded size (KiB) | Compress rate (%) | Decode (msec) |");
    println!("|-|-|-|-|-|");

    for level in 1..=19 {
        let t = Instant::now();
        let encoded = zstd::encode_all(bytes.as_slice(), level).unwrap();
        let enc_ms = t.elapsed().as_secs_f64() * 1000.0;
        let enc_bytes = encoded.len();
        let enc_kb = enc_bytes as f64 / 1024.0;
        let comp_rate = enc_bytes as f64 / bytes.len() as f64 * 100.0;

        let t = Instant::now();
        let decoded = zstd::decode_all(encoded.as_slice()).unwrap();
        let dec_ms = t.elapsed().as_secs_f64() * 1000.0;

        println!("| {level} | {enc_ms} | {enc_kb} | {comp_rate} | {dec_ms} |");
        assert_eq!(bytes, decoded);
    }
}
