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

use std::{collections::BTreeSet, io};

use crate::util::ToVarint;

const PREFIX_LABEL: char = '\r';
const ROOT_LABEL: char = '\n';

#[derive(Default)]
pub struct SuccinctSet {
    leaves: Vec<u64>,
    label_bitmap: Vec<u64>,
    labels: Vec<u8>,
}

impl From<&[String]> for SuccinctSet {
    fn from(value: &[String]) -> Self {
        let mut set = SuccinctSet::default();
        let mut l_idx = 0;
        #[derive(Clone, Copy)]
        struct QElt {
            s: i32,
            e: i32,
            col: i32,
        }
        let mut queue = vec![QElt {
            s: 0,
            e: value.len() as i32,
            col: 0,
        }];
        let mut i = 0;
        while i < queue.len() {
            let mut elt = queue[i].clone();
            if elt.col == value[elt.s as usize].len() as i32 {
                elt.s += 1;
                Self::set_bit(&mut set.leaves, i as isize, 1);
            }
            let mut j = elt.s;
            while j < elt.e {
                let frm = j;
                while j < elt.e
                    && value[j as usize].as_bytes()[elt.col as usize]
                        == value[frm as usize].as_bytes()[elt.col as usize]
                {
                    j += 1
                }
                queue.push(QElt {
                    s: frm,
                    e: j,
                    col: elt.col + 1,
                });
                set.labels
                    .push(value[frm as usize].as_bytes()[elt.col as usize]);
                Self::set_bit(&mut set.label_bitmap, l_idx, 0);
                l_idx += 1;
            }
            Self::set_bit(&mut set.label_bitmap, l_idx, 1);
            l_idx += 1;
            i += 1;
        }
        set
    }
}

impl SuccinctSet {
    fn set_bit(bm: &mut Vec<u64>, i: isize, v: isize) {
        while i >> 6 >= bm.len() as isize {
            bm.push(0);
        }
        bm[(i >> 6) as usize] |= (v as u64) << ((i & 63) as usize);
    }

    pub fn matcher(domains: &BTreeSet<String>, domain_suffix: &BTreeSet<String>) -> Self {
        let mut domain_list: Vec<String> =
            Vec::with_capacity(domains.len() + 2 * domain_suffix.len());
        for domain in domain_suffix {
            if domain.chars().nth(0).unwrap() == '.' {
                domain_list.push(
                    format!("{}{}", PREFIX_LABEL, domain)
                        .chars()
                        .rev()
                        .collect(),
                );
            } else {
                domain_list.push(format!("{}{}", ROOT_LABEL, domain).chars().rev().collect());
            }
        }
        for domain in domains {
            domain_list.push(domain.chars().rev().collect());
        }
        domain_list.sort();
        domain_list.as_slice().into()
    }

    pub fn write<T: io::Write>(&self, writer: &mut T) {
        // Why do we need to write a 0 here? Because that asshole n9i defined a `Reserve` field in `succinctSetData`, which is similar to `succinctSet`; It wasted my entire afternoon!
        writer.write_all(&[0]).unwrap();
        macro_rules! write_data {
            ($field:ident) => {
                writer.write_all(&self.$field.len().to_varint()).unwrap();
                writer
                    .write_all(
                        &self
                            .$field
                            .iter()
                            .flat_map(|v| v.to_be_bytes())
                            .collect::<Vec<u8>>(),
                    )
                    .unwrap();
            };
        }
        write_data!(leaves);
        write_data!(label_bitmap);
        write_data!(labels);
    }
}
