/*
 * This file is part of PROJECT.
 *
 * PROJECT is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * PROJECT is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with PROJECT.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::io::prelude::*;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct BinaryWriter {
    pub filename: String,
}

impl BinaryWriter {

    pub fn new(filename: &str) -> Result<Self, io::Error> {
        let _ = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename);

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

    pub fn write(&mut self, data: &Vec<u8>) -> Result<(), io::Error> {
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
    pub fn open(filename: &str) -> Result<Self, io::Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        Ok(
            Self {
                reader
            }
        )
    }
}

impl Iterator for BinaryReader {
    type Item = Result<Vec<u8>, io::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut line = Vec::new();
        match self.reader.read_until(b'\n', &mut line) {
            Ok(0) => None,
            Ok(_) => {
                line.retain(|&byte| byte != 10 && byte != 13);
                Some(Ok(line))
            },
            Err(e) => Some(Err(e)),
        }
    }
}

