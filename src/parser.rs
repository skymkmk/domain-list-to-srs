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

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    ops::AddAssign,
    path::{Path, PathBuf},
};

use crate::model::Rule;

fn line_trimer<'a>(line: &'a str) -> (Option<&'a str>, &'a str, Option<&'a str>) {
    let comment_pos = line.find("#");
    let line = comment_pos
        .map(|pos| &line[..pos])
        .or(Some(line))
        .unwrap()
        .trim();
    let identifier_end = line.find(":");
    let identifier = identifier_end.map(|pos| line[..pos].trim());
    let content_maybe_with_attribute = identifier_end
        .map(|pos| &line[pos + 1..])
        .or(Some(&line))
        .unwrap()
        .trim();
    let attribute_pos = content_maybe_with_attribute.find("@");
    let attribute = attribute_pos.map(|pos| content_maybe_with_attribute[pos + 1..].trim());
    let content = attribute_pos
        .map(|pos| &content_maybe_with_attribute[..pos])
        .or(Some(content_maybe_with_attribute))
        .unwrap()
        .trim();
    (identifier, content, attribute)
}

pub fn parse_rules(path: &Path) -> HashMap<String, Rule> {
    let domain_file = File::open(path).unwrap();
    let domain_buffer = BufReader::new(domain_file);
    let mut rules: HashMap<String, Rule> = HashMap::new();
    for line in domain_buffer.lines().map(|line| line.unwrap()) {
        let (identifier, content, attribute) = line_trimer(&line);
        if content != "" {
            macro_rules! push_rule {
                ($field: ident) => {
                    rules
                        .entry(String::from("default"))
                        .or_default()
                        .$field
                        .insert(String::from(content));
                    if let Some(attr) = attribute {
                        rules
                            .entry(String::from(attr))
                            .or_default()
                            .$field
                            .insert(String::from(content));
                    }
                };
            }
            match identifier {
                Some(identifier) => match identifier {
                    "include" => {
                        let path: PathBuf = path.parent().unwrap_or(Path::new("/")).join(content);
                        for (k, v) in parse_rules(path.as_path()) {
                            rules.entry(k).or_default().add_assign(v);
                        }
                    }
                    "domain" => {
                        push_rule!(domain_suffix);
                    }
                    "keyword" => {
                        push_rule!(domain_keyword);
                    }
                    "regexp" => {
                        push_rule!(domain_regex);
                    }
                    "full" => {
                        push_rule!(domain);
                    }
                    _ => {
                        eprintln!("unknown identifier {}, ignore it", identifier);
                    }
                },
                None => {
                    push_rule!(domain_suffix);
                }
            }
        }
    }
    rules
}
