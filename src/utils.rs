use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn get_args() -> Result<(File, u32), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [s, proba] => {
            let p = proba.parse::<u32>()?;
            if p == 0 { return Err(Box::from("Frequency cannot be zero. Please enter a positive integer value.")); }

            let path = Path::new(s);
            if !path.is_file() { return Err(Box::from("Path is not a file. Please enter a valid audio file path.")); }

            let file = File::open("src/fart_reverb.aac")?;

            Ok((file, p))
        }
        &_ => Err(Box::from("Incorrect number of arguments\n./executable <File> <Frequency>"))
    }
}