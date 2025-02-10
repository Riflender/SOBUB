use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use rodio::{Decoder, OutputStream, Sink, Source};
use rodio::source::Buffered;

pub fn get_args() -> Result<(File, f64), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_, file_path, proba] => {
            let p = proba.parse::<u32>()?;
            if p == 0 { return Err(Box::from("Frequency cannot be zero. Please enter a positive integer value.")); }

            let path = Path::new(file_path);
            if !path.is_file() { return Err(Box::from("Path is not a file. Please enter a valid audio file path.")); }

            let file = File::open("src/fart_reverb.aac")?;

            Ok((file, 1.0 / p as f64))
        }
        &_ => Err(Box::from("Incorrect number of arguments\n./executable <File> <Frequency>"))
    }
}

pub fn get_rodio_io(file: File) -> Result<(
    OutputStream,
    Buffered<Decoder<BufReader<File>>>,
    Sink
), Box<dyn Error>> {
    // _stream variable is necessary, anonymizing it causes "Error: NoDevice" at Sink creation
    let (stream, stream_handle) = OutputStream::try_default()?;
    let buf = BufReader::new(file);

    let source = Decoder::new(buf)?.buffered();
    let sink = Sink::try_new(&stream_handle)?;

    Ok((stream, source, sink))
}