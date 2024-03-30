// Copyright 2024 Symerlin Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::{fs, io};
use std::collections::HashMap;
use std::path::Path;

#[macro_export]
macro_rules! must_get {
    ($map:expr, $key:expr) => {
        match $map.get(&String::from($key)) {
            Some(v) => {v}
            None => { err!(Error::MissingField, MissingField{ Key: String::from($key) }) }
        }
    };
}

pub fn ReadKeyValueMap(path: &String) -> io::Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    for line in fs::read_to_string(Path::new(path))?.lines() {
        let (key, val) = line.rsplit_once('=').unwrap();
        map.insert(key.to_string(), val.to_string());
    }
    Ok(map)
}
