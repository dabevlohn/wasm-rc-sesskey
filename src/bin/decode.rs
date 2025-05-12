use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use std::fs::File;
use std::io::Write;

fn main() {
    let mut args = std::env::args();
    args.next().expect("No arguments");
    let output_fname = args.next().expect("No arguments");
    let input_string = args.next().expect("No arguments");

    let mut file = File::create(&output_fname).expect("Unable to create file");
    file.write(&URL_SAFE_NO_PAD.decode(&input_string.as_bytes()).unwrap())
        .expect("Unable to write");
}
