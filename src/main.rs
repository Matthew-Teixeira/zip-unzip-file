extern crate flate2;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;
use std::io::ErrorKind;

fn main() {
    if args().len() != 4 {
        eprintln!("Usage: `zip/unzip` `source` `target`");
        return;
    }

    if args().nth(1) == Some("zip".to_string()) {
        zip();
    } else if args().nth(1) == Some("unzip".to_string()) {
        let unzipped_file = unzip();

        match unzipped_file {
            Ok(()) => println!("File unzipped"),
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    println!("{:?}", e.kind())
                }
            },
        }
    }
}

fn zip() {
    let mut input = BufReader::new(File::open(args().nth(2).unwrap()).unwrap());
    let output = File::create(args().nth(3).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!("Source len {:?}", input.get_ref().metadata().unwrap().len());
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed Time: {:?}", start.elapsed());
}

fn unzip() -> std::io::Result<()> {
    // Open the gzipped file
    let input = File::open(args().nth(2).unwrap())?;

    // Create a GzDecoder to read the compressed data
    let mut decoder = GzDecoder::new(BufReader::new(input));

    // Open the output file where the decompressed data will be written
    let mut output = File::create(args().nth(3).unwrap())?;

    // Copy the decompressed data from the decoder to the output file
    copy(&mut decoder, &mut output)?;

    Ok(())
}
