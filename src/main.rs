mod utils;

use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use rand::Rng;

use crate::utils::{get_args, get_rodio_io};

fn main() -> Result<(), Box<dyn Error>> {
    let (file, proba) = get_args()?;
    let (_stream, source, sink) = get_rodio_io(file)?;

    let mut rng = rand::rng();
    let dur = Duration::from_secs(1);

    loop {
        if !rng.random_bool(proba) {
            sleep(dur);
            continue;
        }

        sink.append(source.clone());
        sink.sleep_until_end();
    }
}
