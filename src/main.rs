// Domain List to SRS, a converter that transforms domain-list-community rules
// into SRS.
// Copyright (C) 2025 skymkmk

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License version 3
// as published by the Free Software Foundation.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod model;
mod parser;
mod srs;
mod succinct_set;
mod util;

use std::{
    fs::{self, File},
    io::BufWriter,
    path::Path,
};

use config::Config;

use crate::{model::Setting, parser::parse_rules, srs::write_srs, util::prune_output_dir};

fn main() {
    let setting: Setting = Config::builder()
        .add_source(config::Environment::default())
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap();
    let data_dir = fs::read_dir(&setting.data_path).unwrap();
    prune_output_dir(Path::new(&setting.output_path));
    for file_enrty in data_dir.map(|entry| entry.unwrap()) {
        let path = file_enrty.path();
        let rules = parse_rules(&path);
        let file_name = file_enrty.file_name();
        for (k, v) in rules {
            let mut file_name = if k == "default" {
                file_name.clone()
            } else {
                let mut file_name = file_name.clone();
                file_name.push(format!("@{}", k));
                file_name
            };
            file_name.push(".srs");
            let srs_file = File::create(Path::new(&setting.output_path).join(file_name)).unwrap();
            let mut srs_buffer = BufWriter::new(srs_file);
            write_srs(&mut srs_buffer, &v);
        }
    }
}
