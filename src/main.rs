use std::{env, fs};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use rand::Rng;
use rodio::{Decoder, OutputStream, source::Source};
use tokio::runtime::Runtime;

fn main() {
    let args: &[String] = env::args().collect();
    let ext = [
        OsStr::new("mp3"),
        OsStr::new("wav"),
        OsStr::new("ogg"),
        OsStr::new("oga"),
        OsStr::new("sb0"),
        OsStr::new("flac")
    ];

    let r = match args {
        [s, proba] => {
            let path = Path::new(s);
            let p = proba.parse::<u32>().unwrap();
            if p == 0 {
                panic!()
            }

            match (path.is_file(), path.is_dir()) {
                (true, _) => {
                    if !ext.contains(&path.extension().unwrap()) {
                        panic!()
                    }
                    Ok((Some(path), None, p))
                },
                (_, true) => {
                    let mut v = Vec::new();
                    for file in fs::read_dir(path).unwrap() {
                        if ext.contains(&path.extension().unwrap()) {
                            v.push(file.unwrap().path().as_path());
                        }
                    }
                    if v.len() == 0 {
                        panic!()
                    }
                    Ok((None, Some(v), p))
                },
                _ => panic!(),
            }
        },
        _ => Err("Incorrect number of arguments\n./executable <FileOrDirectory> <Frequency>"),
    }.unwrap();

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut rng = rand::thread_rng();
    let runtime = Runtime::new().unwrap();

    match (r.0, r.1) {
        (Some(path), _) => {
            let mut file = BufReader::new(File::open(path).unwrap());
            loop {
                if rng.gen_range(0..r.2) {
                    let sink = stream_handle.play_once(file.by_ref()).unwrap();
                    sink.sleep_until_end();
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        (_, Some(paths)) => {
            loop {
                if rng.gen_range(0..r.2) {
                    let mut file = BufReader::new(File::open(paths.get(rng.gen_range(0..paths.len()))).unwrap());
                    let source = Decoder::new(file).unwrap();
                    let sink = stream_handle.play_once(file.by_ref()).unwrap();
                    sink.sleep_until_end();
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        _ => panic!(),
    }


    // Play the sound directly on the device
    // stream_handle.play_raw(source.convert_samples()).unwrap();

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    // std::thread::sleep(std::time::Duration::from_secs(5));
}
