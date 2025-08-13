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

use std::{collections::BTreeSet, ops::AddAssign};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Setting {
    pub data_path: String,
    pub output_path: String,
}

pub enum RuleType {
    Domain = 2,
    DomainKeyword = 3,
    DomainRegex = 4,
    DomainFinal = 0xFF,
}

#[derive(Default)]
pub struct Rule {
    pub domain: BTreeSet<String>,
    pub domain_suffix: BTreeSet<String>,
    pub domain_keyword: BTreeSet<String>,
    pub domain_regex: BTreeSet<String>,
}

impl AddAssign for Rule {
    fn add_assign(&mut self, rhs: Self) {
        self.domain.extend(rhs.domain);
        self.domain_suffix.extend(rhs.domain_suffix);
        self.domain_keyword.extend(rhs.domain_keyword);
        self.domain_regex.extend(rhs.domain_regex);
    }
}
