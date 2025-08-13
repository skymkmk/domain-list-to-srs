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

use std::io::{self, Write};

use flate2::{Compression, write::ZlibEncoder};

use crate::{
    model::{Rule, RuleType},
    succinct_set::SuccinctSet,
    util::ToVarint,
};

const MAGIC_BYTES: [u8; 3] = [0x53, 0x52, 0x53];
const VERSION: u8 = 3;

pub fn write_srs<T>(writer: &mut T, value: &Rule)
where
    T: io::Write,
{
    writer.write_all(&MAGIC_BYTES).unwrap();
    writer.write_all(&VERSION.to_be_bytes()).unwrap();
    let mut zwriter = ZlibEncoder::new(writer, Compression::best());
    let varint = 1_usize.to_varint();
    zwriter.write_all(&varint).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    buf.push(0);
    if value.domain.len() > 0 || value.domain_suffix.len() > 0 {
        buf.push(RuleType::Domain as u8);
        let ss = SuccinctSet::matcher(&value.domain, &value.domain_suffix);
        ss.write(&mut buf);
    }
    macro_rules! write_fields {
        ($field:ident, $typ:path) => {
            if value.$field.len() > 0 {
                buf.push($typ as u8);
                buf.extend(value.$field.len().to_varint());
                for rule in value.$field.iter() {
                    buf.extend(rule.len().to_varint());
                    buf.extend(rule.as_bytes());
                }
            }
        };
    }
    write_fields!(domain_keyword, RuleType::DomainKeyword);
    write_fields!(domain_regex, RuleType::DomainRegex);
    buf.push(RuleType::DomainFinal as u8);
    buf.push(0);
    zwriter.write_all(&buf).unwrap();
}
