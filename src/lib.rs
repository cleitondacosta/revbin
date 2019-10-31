use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    input_file: PathBuf,
    output_file: PathBuf,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Usage: revbin INPUT_FILE OUTPUT_FILE");
        }

        let input_file = PathBuf::from(&args[1]);
        let output_file = PathBuf::from(&args[2]);

        if !input_file.is_file() {
            return Err("No such regular file.");
        }

        if output_file.exists() {
            return Err("OUTPUT_FILE already exists.");
        }

        Ok(Config { input_file, output_file })
    }

    pub fn run(&self) -> Result<(), &'static str> {
        let mut input_file_pointer = match File::open(&self.input_file) {
            Ok(file) => file,
            Err(_) => return Err("Could not open INPUT_FILE."),
        };

        let mut output_file_pointer = match File::create(&self.output_file) {
            Ok(file) => file,
            Err(_) => return Err("Could not create OUTPUT_FILE."),
        };

        let mut buffer: [u8; 10] = [0; 10];

        loop {
            let n_bytes_readed = match input_file_pointer.read(&mut buffer) {
                Ok(readed) => readed,
                Err(_) => return Err("Could not read file INPUT_FILE"),
            };

            if n_bytes_readed == 0 {
                break;
            }

            let reverted_bytes = Config::not_bytes(&buffer[..n_bytes_readed]);

            if let Err(_) = output_file_pointer.write_all(&reverted_bytes[..n_bytes_readed]) {
                return Err("Could not write reverted bytes to OUTPUT_FILE");
            }
        }

        Ok(())
    }

    fn not_bytes(bytes: &[u8]) -> [u8; 10] {
        let mut reverted_bytes: [u8; 10] = [0; 10];

        for (i, byte) in bytes.iter().enumerate() {
           reverted_bytes[i] = !byte; 
        }

        reverted_bytes
    }
}
