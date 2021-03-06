// from the previous step
use std::io::{BufWriter, stdout};

use ferris_says::say;

fn main() {
    let stdout = stdout();
    let out = b"Hello fellow Rustaceans!";
    let width = 24;
    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}

