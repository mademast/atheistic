use std::fs::File;

use lzma_rust2::Lzma2Writer;

fn main() {
    // Without this, the LZMA compression runs on every build which makes
    // developement with a langauge server miserable
    println!("cargo::rerun-if-changed=bible.txt");

    let mut bible = File::open("bible.txt").unwrap();
    let lzma_bible = File::create("bible.lzma").unwrap();
    let mut lz = Lzma2Writer::new(lzma_bible, Default::default());

    std::io::copy(&mut bible, &mut lz).unwrap();
}
