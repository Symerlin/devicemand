// Copyright 2024 Symerlin Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::{fs, io};

use err_rs::wrap_result;

use crate::errors::Error;
use crate::keyvalue::ReadKeyValueMap;

pub struct UsbDevice {
    pub Path: String,
    pub DevicePath: String,
}

impl UsbDevice {
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

pub fn ScanUsbDevice() -> Result<Vec<UsbDevice>, Error> {
    let mut devices = vec![];

    for path_ in wrap_result!(Error::IoError, fs::read_dir("/sys/bus/usb/devices/")) {
        let name = wrap_result!(Error::IoError, path_)
            .file_name()
            .into_string()
            .unwrap();
        if name.contains(':') || !name.contains('-') {
            continue;
        }

        let path = format!("/sys/bus/usb/devices/{}", name);

        let uevent = wrap_result!(Error::IoError, ReadKeyValueMap(&format!("{}/uevent", path)));

        devices.push(UsbDevice {
            Path: path.clone(),
            DevicePath: format!("/dev/{}", uevent.get("DEVNAME").unwrap().clone()),
        });
    }

    Ok(devices)
}
