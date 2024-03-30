// Copyright 2024 Symerlin Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

#![allow(nonstandard_style)]
#![allow(unused)]

extern crate core;

use crate::errors::Error;

mod devices;
mod devices_test;
mod rules;
mod uevent;
mod keyvalue;
mod errors;

fn main() -> Result<(), Error> {
    uevent::Serve()
}
