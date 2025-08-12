// Domain List to SRS, a converter that transforms domain-list-community rules into SRS.
// Copyright (C) 2025 skymkmk
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License version 3 as published by the Free Software Foundation.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::{
    fs,
    io::{self, Write},
    path::Path,
};

pub fn prune_output_dir(path: &Path) {
    let metadata = fs::metadata(path);
    match metadata {
        Ok(m) => {
            if m.is_dir() {
                print!(
                    "output dir {} already existed. remove it? [y/N]: ",
                    path.display()
                );
                io::stdout().flush().unwrap();
                let mut action = String::new();
                io::stdin().read_line(&mut action).unwrap();
                if action == "y\n" || action == "Y\n" {
                    fs::remove_dir_all(path).unwrap();
                } else {
                    panic!("output path not empty");
                }
            } else {
                panic!(
                    "output path {} is not a dir and there has already existed a file",
                    path.display()
                );
            }
        }
        Err(e) => {
            if e.kind() != io::ErrorKind::NotFound {
                panic!("{}", e);
            }
        }
    }
    fs::create_dir(path).unwrap();
}

pub trait ToVarint: Copy {
    fn to_varint(self) -> Vec<u8>;
}

macro_rules! impl_to_varint {
    ($typ:ty) => {
        impl ToVarint for $typ {
            fn to_varint(self) -> Vec<u8> {
                let mut value = self;
                let mut buf: Vec<u8> = Vec::new();
                while value >= 0x80 {
                    buf.push((value as u8) | 0x80);
                    value >>= 7;
                }
                buf.push(value as u8);
                buf
            }
        }
    };
}

impl_to_varint!(usize);
