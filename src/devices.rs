// Copyright 2024 Symerlin Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::{fs, io};

pub struct USBDevice {
    pub Path: String,
}

impl USBDevice {
    pub fn GetProductId(&self) -> io::Result<String> {
        let mut s = fs::read_to_string(fs::canonicalize(format!("{}/idProduct", self.Path))?)?;
        s.pop();
        Ok(s)
    }
    pub fn GetVendorId(&self) -> io::Result<String> {
        let mut s = fs::read_to_string(fs::canonicalize(format!("{}/idVendor", self.Path))?)?;
        s.pop();
        Ok(s)
    }
    pub fn GetProductName(&self) -> io::Result<String> {
        let mut s = fs::read_to_string(fs::canonicalize(format!("{}/product", self.Path))?)?;
        s.pop();
        Ok(s)
    }
    pub fn GetVendorName(&self) -> io::Result<String> {
        let mut s = fs::read_to_string(fs::canonicalize(format!("{}/manufacturer", self.Path))?)?;
        s.pop();
        Ok(s)
    }
}

pub fn ScanUSBDevices() -> io::Result<Vec<USBDevice>> {
    let mut devices = vec![];

    for path_ in fs::read_dir("/sys/bus/usb/devices/")? {
        let name = path_?.file_name().into_string().unwrap();
        if name.contains(':') || !name.contains('-') {
            continue;
        }

        devices.push(USBDevice {
            Path: format!("/sys/bus/usb/devices/{}", name),
        });
    }

    Ok(devices)
}
