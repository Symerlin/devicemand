// Copyright 2024 Symerlin Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::fmt::{Debug, Formatter};
use std::num::ParseIntError;
use std::time::SystemTimeError;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    SystemTimeError(SystemTimeError),
    ParseIntError(ParseIntError),
    MissingField(MissingField),
}

pub struct MissingField {
    pub Key: String,
}

impl Debug for MissingField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "missing field: {}", self.Key)
    }
}
