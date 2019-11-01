use std::path::PathBuf;
use std::fs::File;
use std::io;
use std::io::{Read, BufReader};
use std::io::{Write, BufWriter};

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

    pub fn run(&self) -> Result<(), io::Error> {
        let input_file_pointer = File::open(&self.input_file)?;
        let output_file_pointer = File::create(&self.output_file)?;

        let mut reader = BufReader::new(input_file_pointer);
        let mut writer = BufWriter::new(output_file_pointer);

        const BUFFER_SIZE: usize = 32;
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

        loop {
            let n_bytes_readed = reader.read(&mut buffer)?;

            if n_bytes_readed == 0 {
                break;
            }

            let reverted_bytes = Config::not_bytes(&buffer[..n_bytes_readed]);

            writer.write_all(&reverted_bytes)?;
        }

        Ok(())
    }

    pub fn not_bytes(bytes: &[u8]) -> Vec<u8> {
        bytes.iter().map(|byte| !byte).collect()
    }
}
