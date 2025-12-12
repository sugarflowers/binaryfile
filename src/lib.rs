use std::io::prelude::*;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use anyhow::Result;

pub struct BinaryWriter {
    pub filename: String,
}

impl BinaryWriter {

    pub fn new(filename: &str) -> Result<Self> {
        let _ = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;

        Ok(
            BinaryWriter {
                filename: filename.to_string(),
            }
        )
    }

    pub fn open(filename: &str) -> Self {
        BinaryWriter {
            filename: filename.to_string(),
        }
    }

    pub fn write(&mut self, data: &Vec<u8>) -> Result<()> {
        let mut f = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.filename)?;

        f.write_all(data)?;

        Ok(())
    }

}

pub struct BinaryReader {
    pub reader: BufReader<File>,
}

impl BinaryReader {
    pub fn open(filename: &str) -> Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        Ok(
            Self {
                reader
            }
        )
    }
    pub fn read(&mut self) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        self.reader.read_to_end(&mut data)?;
        Ok(data)
    }
}

impl Iterator for BinaryReader {
    type Item = Result<Vec<u8>>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut line = Vec::new();
        match self.reader.read_until(b'\n', &mut line) {
            Ok(0) => None,
            Ok(_) => {
                line.retain(|&byte| byte != 10 && byte != 13);
                Some(Ok(line))
            },
            Err(e) => Some(Err(e.into())),
        }
    }
}

#[test]
fn read_to_all() {
    let br = BinaryReader::open("README2.md").unwrap().read();
    println!("{:?}", br);
}

